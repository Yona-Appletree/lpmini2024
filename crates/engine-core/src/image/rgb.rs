/// RGB image buffer
extern crate alloc;
use alloc::vec::Vec;

pub struct ImageRgb {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // R, G, B interleaved
}

impl ImageRgb {
    pub fn new(width: usize, height: usize) -> Self {
        ImageRgb {
            width,
            height,
            data: alloc::vec![0; width * height * 3],
        }
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> (u8, u8, u8) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) * 3;
            (self.data[idx], self.data[idx + 1], self.data[idx + 2])
        } else {
            (0, 0, 0)
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) * 3;
            self.data[idx] = r;
            self.data[idx + 1] = g;
            self.data[idx + 2] = b;
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
