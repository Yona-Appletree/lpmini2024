/// Trait for nodes in the scene graph.
///
/// Nodes must implement `RecordValue` from `lp-data` so their properties
/// (input, state, output) can be accessed.
pub trait LpNode: lp_data::kind::record::record_value::RecordValue {
    /// Update the node's internal state and output based on the current context.
    ///
    /// This method mutates the node's internal output field.
    fn update(&mut self, context: &dyn NodeContext) -> Result<(), lp_data::RuntimeError>;
}

/// Context provided to nodes during update.
pub trait NodeContext {
    /// Get the current frame time in milliseconds.
    fn frame_time_ms(&self) -> i64;

    /// Get the output of another node by ID (for future input resolution).
    ///
    /// Returns None if the node doesn't exist or hasn't been computed yet.
    fn get_node_output(&self, _node_id: &str) -> Option<lp_data::kind::value::LpValueRef> {
        None
    }
}
