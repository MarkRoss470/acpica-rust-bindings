//! # Dependencies
//! This crate builds ACPICA from source, which means that the following system commands must be available:
//! * GCC for compiling C code
//! * AR for creating a static library

#![no_std]
#![feature(c_variadic)]
#![feature(pointer_byte_offsets)]
// Safety best practises
#![warn(unsafe_op_in_unsafe_fn)]
#![deny(clippy::missing_safety_doc)]
// Public API best practises
#![deny(
    missing_debug_implementations,
    // missing_docs,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]
#![warn(clippy::all, clippy::correctness)]
// #![warn(clippy::pedantic)]
#![deny(improper_ctypes, improper_ctypes_definitions)]

extern crate alloc;

mod bindings;
pub mod interface;
pub mod types;
