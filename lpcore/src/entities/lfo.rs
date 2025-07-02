use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct Input {
    #[schemars(description = "Period of oscillation")]
    #[schemars(extend("ui" = "123"))]
    period_ms: f64,

    #[serde(default)]
    waveform: LfoWaveform,

    #[serde(default)]
    min: f64,

    #[serde(default = "default_max")]
    max: f64,
}

fn default_max() -> f64 {
    1.0
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
enum LfoWaveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Default for LfoWaveform {
    fn default() -> Self {
        Self::Sine
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct State {
    offset_ms: f64,
}

pub fn schema() -> Schema {
    schema_for!(Input)
}
