pub mod lfo_input;
pub mod lfo_math;
pub mod lfo_node;
pub mod lfo_state;
pub mod lfo_waveform;

// LfoInput and LfoWaveform are defined in lfo_node.rs (along with LfoNode)
// because the proc macro needs them in the same module to find shape constants
pub use lfo_input::LfoInput;
// LfoInput and LfoWaveform are defined in lfo_node.rs (along with LfoNode)
// because the proc macro needs them in the same module to find shape constants
pub use lfo_node::LfoNode;
// LfoInput and LfoWaveform are defined in lfo_node.rs (along with LfoNode)
// because the proc macro needs them in the same module to find shape constants
pub use lfo_state::LfoState;
// LfoInput and LfoWaveform are defined in lfo_node.rs (along with LfoNode)
// because the proc macro needs them in the same module to find shape constants
pub use lfo_waveform::LfoWaveform;
