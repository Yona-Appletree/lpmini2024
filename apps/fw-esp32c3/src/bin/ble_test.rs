#![no_std]
#![no_main]

extern crate alloc;

use core::convert::Infallible;

use bt_hci::controller::ExternalController;
use bt_hci::transport::{Error as HciTransportError, Transport, WithIndicator};
use bt_hci::{ControllerToHostPacket, HostToControllerPacket};
use defmt::{error, info, warn};
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Instant;
use esp_hal::clock::CpuClock;
use esp_hal::rng::Rng;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use heapless::{String, Vec};
use serde::{Deserialize, Serialize};
use static_cell::StaticCell;
use trouble_host::prelude::*;
use {panic_rtt_target as _, serde_json_core as json};

esp_bootloader_esp_idf::esp_app_desc!();

/// Maximum payload size we accept from the Web Bluetooth client.
const MAX_PAYLOAD: usize = 180;
/// Maximum serialized JSON bytes we support per exchange.
const MAX_JSON: usize = 256;

const SERVICE_UUID: &str = "0000ff00-0000-1000-8000-00805f9b34fb";
const CHARACTERISTIC_UUID: &str = "0000ff01-0000-1000-8000-00805f9b34fb";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "payload")]
pub enum Message {
    Echo { text: String<MAX_PAYLOAD> },
    Ping,
    Status { uptime_ms: u64 },
    Error { message: String<MAX_PAYLOAD> },
}

impl Message {
    pub fn encode(&self) -> Result<Vec<u8, MAX_JSON>, json::ser::Error> {
        let mut buf: Vec<u8, MAX_JSON> = Vec::new();
        json::to_vec(self, &mut buf)?;
        Ok(buf)
    }

    pub fn decode(raw: &[u8]) -> Result<Self, MessageDecodeError> {
        let slice = core::str::from_utf8(raw).map_err(|_| MessageDecodeError::Utf8)?;
        json::from_str(slice)
            .map(|(msg, _)| msg)
            .map_err(|_| MessageDecodeError::Json)
    }
}

#[derive(Debug)]
pub enum MessageDecodeError {
    Utf8,
    Json,
}

struct BleTransport<'d> {
    connector: Mutex<NoopRawMutex, BleConnector<'d>>,
}

impl<'d> BleTransport<'d> {
    fn new(connector: BleConnector<'d>) -> Self {
        Self {
            connector: Mutex::new(connector),
        }
    }
}

impl<'d> embedded_io::ErrorType for BleTransport<'d> {
    type Error = HciTransportError<BleConnectorError>;
}

impl<'d> Transport for BleTransport<'d> {
    async fn read<'a>(&self, buf: &'a mut [u8]) -> Result<ControllerToHostPacket<'a>, Self::Error> {
        let mut guard = self.connector.lock().await;
        ControllerToHostPacket::read_hci_async(&mut *guard, buf)
            .await
            .map_err(HciTransportError::Read)
    }

    async fn write<T: HostToControllerPacket>(&self, packet: &T) -> Result<(), Self::Error> {
        let mut guard = self.connector.lock().await;
        WithIndicator::new(packet)
            .write_hci_async(&mut *guard)
            .await
            .map_err(HciTransportError::Write)
    }
}

type Controller<'d> = ExternalController<BleTransport<'d>, 4>;

static HOST_RESOURCES: StaticCell<HostResources<1, 3, 256>> = StaticCell::new();

#[gatt_service(uuid = CHARACTERISTIC_UUID)]
struct EchoService {
    #[characteristic(uuid = CHARACTERISTIC_UUID, read, write, notify)]
    value: Vec<u8, MAX_JSON>,
}

#[gatt_server(mutex_type = NoopRawMutex, attribute_table_size = 24)]
struct AppServer {
    echo: EchoService,
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();
    info!("Starting BLE echo test");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // embassy systimer setup
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let mut rng = Rng::new(peripherals.RNG);
    let mut address_bytes = [0u8; 6];
    for chunk in address_bytes.iter_mut() {
        *chunk = rng.random() as u8;
    }
    address_bytes[5] |= 0b1100_0000;

    let tg0 = TimerGroup::new(peripherals.TIMG0);
    let wifi_controller = esp_wifi::init(tg0.timer0, rng).expect("esp_wifi init failed");

    let ble_connector = BleConnector::new(&wifi_controller, peripherals.BT);
    let transport = BleTransport::new(ble_connector);
    let controller: Controller<'_> = ExternalController::new(transport);

    let resources = HOST_RESOURCES.init(HostResources::new());
    let stack =
        trouble_host::new(controller, resources).set_random_address(Address::random(address_bytes));
    let Host {
        mut peripheral,
        mut runner,
        ..
    } = stack.build();

    let gap = GapConfig::Peripheral(PeripheralConfig {
        name: "LP BLE Echo",
        appearance: &appearance::UNKNOWN,
    });
    let server = AppServer::new_with_config(gap).expect("Failed to create GATT server");

    if let Ok(initial) = (Message::Status { uptime_ms: 0 }).encode() {
        let _ = server.echo.value.set(&server.server, &initial);
    }

    let start = Instant::now();
    match select(
        runner.run(),
        peripheral_loop(&mut peripheral, &server, start),
    )
    .await
    {
        Either::First(res) => {
            if let Err(e) = res {
                error!("BLE host runner terminated: {:?}", defmt::Debug2Format(&e));
            }
        }
        Either::Second(_) => unreachable!(),
    }
}

async fn peripheral_loop<'srv>(
    peripheral: &mut Peripheral<'srv, Controller<'srv>>,
    server: &AppServer<'srv>,
    start: Instant,
) -> Infallible {
    let mut adv_data = [0u8; 31];
    let mut scan_data = [0u8; 31];

    AdStructure::encode_slice(
        &[AdStructure::Flags(
            LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED,
        )],
        &mut adv_data,
    )
    .ok();
    AdStructure::encode_slice(
        &[AdStructure::CompleteLocalName(b"LP BLE Echo")],
        &mut scan_data,
    )
    .ok();

    loop {
        let advertisement = Advertisement::ConnectableScannableUndirected {
            adv_data: &adv_data,
            scan_data: &scan_data,
        };

        match peripheral
            .advertise(&Default::default(), advertisement)
            .await
        {
            Ok(acceptor) => match acceptor.accept().await {
                Ok(conn) => match conn.with_attribute_server(&server.server) {
                    Ok(gatt) => {
                        info!("Client connected");
                        handle_connection(gatt, server, start).await;
                        info!("Client disconnected");
                    }
                    Err(err) => {
                        error!(
                            "Failed to bind GATT server: {:?}",
                            defmt::Debug2Format(&err)
                        );
                    }
                },
                Err(err) => {
                    error!(
                        "Failed to accept connection: {:?}",
                        defmt::Debug2Format(&err)
                    );
                }
            },
            Err(err) => error!("Advertise error: {:?}", defmt::Debug2Format(&err)),
        }
    }
}

async fn handle_connection<'srv>(
    connection: GattConnection<'srv, 'srv>,
    server: &AppServer<'srv>,
    start: Instant,
) {
    loop {
        match connection.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("Disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => match event {
                Ok(GattEvent::Write(write)) => {
                    process_write(write, &connection, server, start).await;
                }
                Ok(GattEvent::Read(read)) => match read.accept() {
                    Ok(reply) => reply.send().await,
                    Err(err) => {
                        error!("Read reply failed: {:?}", defmt::Debug2Format(&err));
                    }
                },
                Err(err) => {
                    error!("Gatt error: {:?}", defmt::Debug2Format(&err));
                }
            },
            GattConnectionEvent::ConnectionParamsUpdated { .. } => {}
            GattConnectionEvent::PhyUpdated { .. } => {}
        }
    }
}

async fn process_write<'srv>(
    write: WriteEvent<'srv, 'srv>,
    connection: &GattConnection<'srv, 'srv>,
    server: &AppServer<'srv>,
    start: Instant,
) {
    match Message::decode(write.data()) {
        Ok(message) => {
            let response = build_response(message, start);
            match response.encode() {
                Ok(payload) => {
                    if let Err(err) = server.echo.value.set(&server.server, &payload) {
                        error!("Failed to update value: {:?}", defmt::Debug2Format(&err));
                    }
                    match write.accept() {
                        Ok(reply) => reply.send().await,
                        Err(err) => {
                            error!("Failed to ack write: {:?}", defmt::Debug2Format(&err));
                            return;
                        }
                    }
                    if let Err(err) = server.echo.value.notify(connection, &payload).await {
                        warn!("Notify failed: {:?}", defmt::Debug2Format(&err));
                    }
                }
                Err(_) => {
                    error!("Failed to encode response");
                    match write.reject(AttErrorCode::UNLIKELY_ERROR) {
                        Ok(reply) => reply.send().await,
                        Err(err) => {
                            error!("Failed to reject write: {:?}", defmt::Debug2Format(&err))
                        }
                    }
                }
            }
        }
        Err(_) => {
            warn!("Invalid message received");
            match write.reject(AttErrorCode::VALUE_NOT_ALLOWED) {
                Ok(reply) => reply.send().await,
                Err(err) => error!("Failed to reject write: {:?}", defmt::Debug2Format(&err)),
            }
        }
    }
}

fn build_response(message: Message, start: Instant) -> Message {
    match message {
        Message::Echo { text } => Message::Echo { text },
        Message::Ping | Message::Status { .. } => Message::Status {
            uptime_ms: start.elapsed().as_millis() as u64,
        },
        Message::Error { message } => Message::Error { message },
    }
}
