/// Waveforms for low frequency oscillators.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    lp_data_derive::EnumValue,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum LfoWaveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Default for LfoWaveform {
    fn default() -> Self {
        LfoWaveform::Sine
    }
}
