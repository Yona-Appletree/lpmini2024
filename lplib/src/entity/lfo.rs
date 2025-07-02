use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
enum LfoWaveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

struct LfoInput {
    period: f64,
    waveform: LfoWaveform,
}


