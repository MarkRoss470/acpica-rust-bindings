//! # Dependencies
//! This crate builds ACPICA from source, which means that the following system commands must be available:
//! * GCC for compiling C code
//! * AR for creating a static library

#![no_std]
#![feature(c_variadic)]
#![feature(pointer_byte_offsets)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(clippy::all, clippy::correctness)]
// #![warn(clippy::pedantic)]
#![deny(improper_ctypes, improper_ctypes_definitions)]

use interface::debug_trace;

extern crate alloc;

mod bindings;
pub mod types;
pub mod interface;

#[test]
fn test() {
    debug_trace("test", 0, 0, 0);
}
