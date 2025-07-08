#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(no_std))]

extern crate core;

pub mod data;
mod entities;
mod entity;
pub mod expr;
pub mod module;
pub mod scene;
