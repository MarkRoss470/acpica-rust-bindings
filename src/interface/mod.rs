pub mod handler;

use core::sync::atomic::Ordering;

use alloc::ffi::CString;

use crate::bindings::{functions::AcpiDebugTrace, types::FfiAcpiGenericAddress};

use self::{status::AcpiError, handler::SUBSYSTEM_IS_INITIALIZED};

pub mod object;
pub mod status;

///  GAS - Generic Address Structure (ACPI 2.0+)
/// 
///  Note: Since this structure is used in the ACPI tables, it is byte aligned.
///  If misaligned access is not supported by the hardware, accesses to the
///  64-bit Address field must be performed with care.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiGenericAddress {
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
}

impl From<FfiAcpiGenericAddress> for AcpiGenericAddress {
    fn from(value: FfiAcpiGenericAddress) -> Self {
        Self {
            space_id: value.SpaceId,
            bit_width: value.BitWidth,
            bit_offset: value.BitOffset,
            access_width: value.AccessWidth,
            address: value.Address,
        }
    }
}

/// Rust binding to the ACPICA `AcpiDebugTrace` function.
/// 
/// # Panics
/// * If the OS interface has not been set up using [`register_interface`][crate::os_interface::register_interface]
/// * If `name` contains null bytes, including at the end.
/// 
/// TODO: Find enums for layer, level, and flags
pub fn debug_trace(name: &str, level: u32, layer: u32, flags: u32) -> Result<(), AcpiError> {
    if !SUBSYSTEM_IS_INITIALIZED.load(Ordering::Relaxed) {
        panic!("Subsystem not initialised")
    }

    let ffi_name = CString::new(name).expect("name should not contain null bytes");

    unsafe { AcpiDebugTrace(ffi_name.as_ptr(), level, layer, flags).as_result() }
}
