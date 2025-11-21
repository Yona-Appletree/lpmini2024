/// Grayscale image buffer
extern crate alloc;
use alloc::vec::Vec;

use lp_script::dec32::Dec32;

pub struct ImageGrey {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Dec32>,
}

impl ImageGrey {
    pub fn new(width: usize, height: usize) -> Self {
        ImageGrey {
            width,
            height,
            data: alloc::vec![Dec32::ZERO; width * height],
        }
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Dec32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            Dec32::ZERO
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, value: Dec32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[Dec32] {
        &self.data
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [Dec32] {
        &mut self.data
    }
}
