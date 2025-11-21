/// Shader reference wrapper
///
/// Opaque handle to a compiled shader program managed by a GfxContext.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderRef {
    id: u32,
}

impl ShaderRef {
    /// Create a new shader reference from an ID
    pub fn new(id: u32) -> Self {
        ShaderRef { id }
    }

    /// Get the underlying ID
    pub fn id(&self) -> u32 {
        self.id
    }
}
