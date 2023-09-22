use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  PMTT - Platform Memory Topology Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_pmtt {
    pub Header: FfiAcpiTableHeader,
    pub MemoryDeviceCount: u32,
}
///  PMTT - Platform Memory Topology Table (ACPI 5.0)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_header {
    pub Type: u8,
    pub Reserved1: u8,
    pub Length: u16,
    pub Flags: u16,
    pub Reserved2: u16,
    pub MemoryDeviceCount: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_socket {
    pub Header: acpi_pmtt_header,
    pub SocketId: u16,
    pub Reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_controller {
    pub Header: acpi_pmtt_header,
    pub ControllerId: u16,
    pub Reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_physical_component {
    pub Header: acpi_pmtt_header,
    pub BiosHandle: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct acpi_pmtt_vendor_specific {
    pub Header: acpi_pmtt_header,
    pub TypeUuid: [u8; 16usize],
    Specific: __IncompleteArrayField<u8>,
}
