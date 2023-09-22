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

#![allow(clippy::redundant_static_lifetimes, unsafe_op_in_unsafe_fn)]

extern crate alloc;

mod bindings;
pub mod types;
pub mod interface;