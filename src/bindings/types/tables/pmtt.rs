use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  PMTT - Platform Memory Topology Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePmtt {
    pub header: FfiAcpiTableHeader,
    pub memory_device_count: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttHeader {
    pub header_type: u8,
    pub reserved1: u8,
    pub length: u16,
    pub flags: u16,
    pub reserved2: u16,
    pub memory_device_count: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttSocket {
    pub header: FfiAcpiPmttHeader,
    pub socket_id: u16,
    pub reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttController {
    pub header: FfiAcpiPmttHeader,
    pub controller_id: u16,
    pub reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttPhysicalComponent {
    pub header: FfiAcpiPmttHeader,
    pub bios_handle: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FfiAcpiPmttVendorSpecific {
    pub header: FfiAcpiPmttHeader,
    pub type_uuid: [u8; 16usize],
    specific: __IncompleteArrayField<u8>,
}
