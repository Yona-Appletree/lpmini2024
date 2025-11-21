<script lang="ts">
  import { onDestroy } from 'svelte';

  const SERVICE_UUID = '0000ff00-0000-1000-8000-00805f9b34fb';
  const CHARACTERISTIC_UUID = '0000ff01-0000-1000-8000-00805f9b34fb';
  const MAX_PAYLOAD = 180;

  type Direction = 'sent' | 'received';
  type LogEntry = {
    direction: Direction;
    timestamp: Date;
    raw: string;
    parsed?: Record<string, unknown>;
  };

  let device: BluetoothDevice | null = null;
  let characteristic: BluetoothRemoteGATTCharacteristic | null = null;
  let isConnecting = false;
  let isConnected = false;
  let statusMessage = 'Disconnected';
  let messageText = '';
  let logs: LogEntry[] = [];

  const textEncoder = new TextEncoder();
  const textDecoder = new TextDecoder();

  function appendLog(direction: Direction, raw: string) {
    let parsed: Record<string, unknown> | undefined;
    try {
      parsed = JSON.parse(raw) as Record<string, unknown>;
    } catch {
      parsed = undefined;
    }
    logs = [
      ...logs,
      {
        direction,
        timestamp: new Date(),
        raw,
        parsed,
      },
    ];
  }

  async function connect() {
    if (!navigator.bluetooth) {
      statusMessage = 'Web Bluetooth API is not available in this browser.';
      return;
    }

    isConnecting = true;
    statusMessage = 'Scanning for LP BLE device…';

    try {
      device = await navigator.bluetooth.requestDevice({
        filters: [{ services: [SERVICE_UUID] }],
        optionalServices: [SERVICE_UUID],
      });

      device.addEventListener('gattserverdisconnected', handleDisconnect);

      const server = await device.gatt?.connect();
      if (!server) {
        throw new Error('Failed to open GATT server');
      }

      const service = await server.getPrimaryService(SERVICE_UUID);
      characteristic = await service.getCharacteristic(CHARACTERISTIC_UUID);

      characteristic.addEventListener('characteristicvaluechanged', handleNotification);
      await characteristic.startNotifications();

      isConnected = true;
      statusMessage = `Connected to ${device.name ?? 'LP BLE Echo'}`;
    } catch (error) {
      console.error('BLE connection failed', error);
      statusMessage =
        error instanceof Error ? `Connection failed: ${error.message}` : 'Connection failed';
      await disconnect();
    } finally {
      isConnecting = false;
    }
  }

  async function disconnect() {
    if (characteristic) {
      try {
        await characteristic.stopNotifications();
      } catch (error) {
        console.warn('stopNotifications failed', error);
      }
      characteristic.removeEventListener('characteristicvaluechanged', handleNotification);
      characteristic = null;
    }

    if (device) {
      device.removeEventListener('gattserverdisconnected', handleDisconnect);
      if (device.gatt?.connected) {
        device.gatt.disconnect();
      }
    }

    device = null;
    isConnected = false;
    statusMessage = 'Disconnected';
  }

  function handleDisconnect() {
    statusMessage = 'Device disconnected';
    void disconnect();
  }

  async function sendEcho() {
    if (!characteristic) {
      statusMessage = 'Connect to the device first.';
      return;
    }
    const text = messageText.trim();
    if (!text) {
      statusMessage = 'Enter a message to echo.';
      return;
    }
    if (text.length > MAX_PAYLOAD) {
      statusMessage = `Message too long (max ${MAX_PAYLOAD} characters).`;
      return;
    }
    const payload = {
      type: 'Echo',
      payload: { text },
    };
    await writePayload(payload);
    messageText = '';
  }

  async function sendPing() {
    if (!characteristic) {
      statusMessage = 'Connect to the device first.';
      return;
    }
    const payload = {
      type: 'Ping',
    };
    await writePayload(payload);
  }

  async function writePayload(payload: Record<string, unknown>) {
    if (!characteristic) {
      return;
    }
    try {
      const raw = JSON.stringify(payload);
      appendLog('sent', raw);
      await characteristic.writeValue(textEncoder.encode(raw));
      statusMessage = 'Message sent.';
    } catch (error) {
      console.error('Failed to write characteristic', error);
      statusMessage = error instanceof Error ? `Write failed: ${error.message}` : 'Write failed.';
    }
  }

  function handleNotification(event: Event) {
    const target = event.target as BluetoothRemoteGATTCharacteristic;
    const value = target.value;
    if (!value) {
      return;
    }
    const buffer = new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
    const raw = textDecoder.decode(buffer);
    appendLog('received', raw);
  }

  onDestroy(() => {
    void disconnect();
  });
</script>

<svelte:head>
  <title>BLE Echo Test</title>
</svelte:head>

<section class="page">
  <header>
    <h1>BLE Echo Test</h1>
    <p class="status">
      Status: <span class:is-connected={isConnected}>{statusMessage}</span>
    </p>
  </header>

  <div class="controls">
    <button on:click={connect} disabled={isConnecting || isConnected}>
      {isConnecting ? 'Connecting…' : 'Connect'}
    </button>
    <button on:click={disconnect} disabled={!isConnected}> Disconnect </button>
  </div>

  <form
    class="message-form"
    on:submit|preventDefault={async () => {
      await sendEcho();
    }}
  >
    <label>
      Echo Text
      <input
        type="text"
        bind:value={messageText}
        maxlength={MAX_PAYLOAD}
        placeholder="Message to echo"
        disabled={!isConnected || isConnecting}
      />
    </label>
    <div class="message-actions">
      <button type="submit" disabled={!isConnected || isConnecting || !messageText.trim()}>
        Send Echo
      </button>
      <button type="button" on:click={sendPing} disabled={!isConnected || isConnecting}>
        Send Ping
      </button>
    </div>
  </form>

  <section class="log">
    <h2>Message Log</h2>
    {#if logs.length === 0}
      <p>No messages yet.</p>
    {:else}
      <ul>
        {#each logs.slice().reverse() as entry (entry.timestamp.getTime() + entry.direction)}
          <li
            class:received={entry.direction === 'received'}
            class:sent={entry.direction === 'sent'}
          >
            <div class="meta">
              <span class="direction">{entry.direction}</span>
              <time datetime={entry.timestamp.toISOString()}>
                {entry.timestamp.toLocaleTimeString()}
              </time>
            </div>
            <pre>{entry.parsed ? JSON.stringify(entry.parsed, null, 2) : entry.raw}</pre>
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</section>

<style>
  .page {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 720px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
  }

  header h1 {
    font-size: 1.75rem;
    margin: 0;
  }

  .status span {
    font-weight: 600;
  }

  .status span.is-connected {
    color: var(--color-success, #058e3f);
  }

  .controls {
    display: flex;
    gap: 1rem;
  }

  .controls button {
    min-width: 8rem;
  }

  .message-form {
    display: grid;
    gap: 0.75rem;
  }

  .message-form label {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    font-weight: 600;
  }

  .message-form input {
    padding: 0.6rem;
    border-radius: 0.5rem;
    border: 1px solid var(--outline, #ccc);
  }

  .message-actions {
    display: flex;
    gap: 0.75rem;
  }

  .message-actions button {
    flex: 0 0 auto;
  }

  .log ul {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .log li {
    border-radius: 0.75rem;
    padding: 0.75rem;
    border: 1px solid var(--outline, #d0d0d0);
    background: var(--surface, #fdfdfd);
  }

  .log li.sent {
    border-color: rgba(46, 134, 222, 0.4);
  }

  .log li.received {
    border-color: rgba(5, 142, 63, 0.4);
  }

  .meta {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    margin-bottom: 0.4rem;
  }

  .direction {
    text-transform: uppercase;
    font-weight: 700;
    letter-spacing: 0.06em;
  }

  pre {
    margin: 0;
    white-space: pre-wrap;
    font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
  }
</style>
