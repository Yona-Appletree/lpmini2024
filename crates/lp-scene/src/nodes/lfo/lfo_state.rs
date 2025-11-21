/// Internal state for LFO node.
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize,
)]
pub struct LfoState {
    /// Offset in milliseconds to maintain phase when period changes.
    pub offset_ms: i32,

    /// Previous period value to detect changes.
    pub prev_period_ms: i32,
}

impl Default for LfoState {
    fn default() -> Self {
        Self {
            offset_ms: 0,
            prev_period_ms: 0,
        }
    }
}
