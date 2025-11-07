/// Grid and serpentine LED mappings
use super::{LedMap, LedMapping};

impl LedMapping {
    /// Create a simple grid mapping (for testing)
    /// Maps 128 LEDs to an 16x8 grid in row-major order
    pub fn grid_16x8() -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        for i in 0..128 {
            let x = i % 16;
            let y = i / 16;
            maps[i] = LedMap::new(x, y);
        }
        LedMapping { maps }
    }

    /// Create a serpentine/zigzag mapping (common for LED matrices)
    /// Even rows go left-to-right, odd rows go right-to-left
    pub fn serpentine_16x8() -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        for i in 0..128 {
            let y = i / 16;
            let x = if y % 2 == 0 { i % 16 } else { 15 - (i % 16) };
            maps[i] = LedMap::new(x, y);
        }
        LedMapping { maps }
    }
}
