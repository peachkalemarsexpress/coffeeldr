#![doc = include_str!("../README.md")]
#![allow(internal_features)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]

#[cfg(target_os = "windows")]
mod beacon;
mod parse;
mod error;

/// Module exposing the `BeaconPack` structure for packing and manipulating binary data, strings, integers, and buffers.
mod beacon_pack;
pub use beacon_pack::*;

/// Module containing the code that will load the COFF
#[cfg(target_os = "windows")]
mod loader;
#[cfg(target_os = "windows")]
pub use loader::*;
