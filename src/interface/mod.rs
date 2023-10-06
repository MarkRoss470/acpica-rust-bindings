pub mod handler;

use core::sync::atomic::Ordering;

use alloc::ffi::CString;

use crate::bindings::functions::AcpiDebugTrace;

use self::{handler::SUBSYSTEM_IS_INITIALIZED, status::AcpiError};

pub mod object;
pub mod status;
pub mod types;

/// Rust binding to the ACPICA `AcpiDebugTrace` function.
///
/// # Panics
/// * If the OS interface has not been set up using [`register_interface`]
/// * If `name` contains null bytes, including at the end.
///
/// TODO: Find enums for layer, level, and flags
/// 
/// [`register_interface`]: handler::register_interface
pub fn debug_trace(name: &str, level: u32, layer: u32, flags: u32) -> Result<(), AcpiError> {
    assert!(SUBSYSTEM_IS_INITIALIZED.load(Ordering::Relaxed), "Subsystem not initialised");

    let ffi_name = CString::new(name).expect("name should not contain null bytes");

    // SAFETY: The passed pointer is valid as it was taken from a CString
    unsafe { AcpiDebugTrace(ffi_name.as_ptr(), level, layer, flags).as_result() }
}
