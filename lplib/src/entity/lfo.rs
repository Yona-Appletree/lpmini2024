use crate::dynval::schema::SchemaNode;

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

trait Dynval<T> {
    
}