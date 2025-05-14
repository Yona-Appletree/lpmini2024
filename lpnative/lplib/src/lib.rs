#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(no_std))]

extern crate core;

pub mod glsl;
pub mod shaders;

pub fn compute(value: u32) -> u32 {
    value * 2
}