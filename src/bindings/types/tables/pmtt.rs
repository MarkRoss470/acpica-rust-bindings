use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  PMTT - Platform Memory Topology Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePmtt {
    pub Header: FfiAcpiTableHeader,
    pub MemoryDeviceCount: u32,
}
///  PMTT - Platform Memory Topology Table (ACPI 5.0)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttHeader {
    pub Type: u8,
    pub Reserved1: u8,
    pub Length: u16,
    pub Flags: u16,
    pub Reserved2: u16,
    pub MemoryDeviceCount: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttSocket {
    pub Header: FfiAcpiPmttHeader,
    pub SocketId: u16,
    pub Reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttController {
    pub Header: FfiAcpiPmttHeader,
    pub ControllerId: u16,
    pub Reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPmttPhysicalComponent {
    pub Header: FfiAcpiPmttHeader,
    pub BiosHandle: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FfiAcpiPmttVendorSpecific {
    pub Header: FfiAcpiPmttHeader,
    pub TypeUuid: [u8; 16usize],
    Specific: __IncompleteArrayField<u8>,
}
