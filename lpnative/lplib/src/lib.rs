#![cfg_attr(not(feature = "std"), no_std)]

pub fn compute(value: u32) -> u32 {
    value * 2
}