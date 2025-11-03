/// Mapping configuration types
use super::LedMapping;

/// Configuration for different mapping types
#[derive(Clone)]
pub enum MappingConfig {
    Grid16x8,
    Serpentine16x8,
    Spiral {
        arms: usize,
        width: usize,
        height: usize,
    },
    CircularPanel {
        ring_counts: alloc::vec::Vec<usize>,
        center_x: f32,
        center_y: f32,
        max_radius: f32,
    },
    CircularPanel7Ring {
        width: usize,
        height: usize,
    },
}

impl MappingConfig {
    /// Build the actual LedMapping from this config
    pub fn build(&self) -> LedMapping {
        match self {
            MappingConfig::Grid16x8 => LedMapping::grid_16x8(),
            MappingConfig::Serpentine16x8 => LedMapping::serpentine_16x8(),
            MappingConfig::Spiral { arms, width, height } => {
                LedMapping::spiral(*arms, *width, *height)
            }
            MappingConfig::CircularPanel { ring_counts, center_x, center_y, max_radius } => {
                LedMapping::circular_panel(ring_counts, *center_x, *center_y, *max_radius)
            }
            MappingConfig::CircularPanel7Ring { width, height } => {
                LedMapping::circular_panel_7ring(*width, *height)
            }
        }
    }
}

