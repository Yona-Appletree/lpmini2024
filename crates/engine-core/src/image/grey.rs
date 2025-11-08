/// Grayscale image buffer
extern crate alloc;
use alloc::vec::Vec;

use lp_script::fixed::Fixed;

pub struct ImageGrey {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Fixed>,
}

impl ImageGrey {
    pub fn new(width: usize, height: usize) -> Self {
        ImageGrey {
            width,
            height,
            data: alloc::vec![Fixed::ZERO; width * height],
        }
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Fixed {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            Fixed::ZERO
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, value: Fixed) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[Fixed] {
        &self.data
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [Fixed] {
        &mut self.data
    }
}
