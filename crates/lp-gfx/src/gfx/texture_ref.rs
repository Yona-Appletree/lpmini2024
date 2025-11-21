/// Texture reference wrapper
///
/// Opaque handle to a texture managed by a GfxContext.
/// The ID matches OpenGL texture ID convention (u32).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TexRef {
    id: u32,
}

impl TexRef {
    /// Create a new texture reference from an ID
    pub fn new(id: u32) -> Self {
        TexRef { id }
    }

    /// Get the underlying ID
    pub fn id(&self) -> u32 {
        self.id
    }
}
