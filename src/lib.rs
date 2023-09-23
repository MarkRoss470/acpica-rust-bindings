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
    missing_docs,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]
// Correctness lints
#![warn(clippy::all, clippy::correctness)]
// FFI correctness
#![deny(improper_ctypes, improper_ctypes_definitions)]
// Pedantic lints
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

extern crate alloc;

mod bindings;
mod interface;

pub use interface::*;

/// If no code in the crate calls ACPICA functions when compiling tests, cargo will not link the ACPICA library.
/// This means that some linker errors will go silently unnoticed until the crate is compiled outside of tests.
/// This test forces ACPICA to be linked to find these errors.
#[test]
#[ignore = "This just forces the compiler to actually link in ACPICA"]
#[allow(unreachable_code)]
fn force_link() {
    panic!("This test should not be run,just compiled");
    debug_trace("Hello world", 0, 0, 0).unwrap();
}