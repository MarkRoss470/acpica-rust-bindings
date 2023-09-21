use crate::{types::AcpiTableHeader, bindings::types::__IncompleteArrayField};


#[doc = " PMTT - Platform Memory Topology Table (ACPI 5.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_pmtt {
    pub Header: AcpiTableHeader,
    pub MemoryDeviceCount: u32,
}
#[doc = " PMTT - Platform Memory Topology Table (ACPI 5.0)"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_PMTT = acpi_table_pmtt;
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
pub type ACPI_PMTT_HEADER = acpi_pmtt_header;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_socket {
    pub Header: ACPI_PMTT_HEADER,
    pub SocketId: u16,
    pub Reserved: u16,
}
pub type ACPI_PMTT_SOCKET = acpi_pmtt_socket;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_controller {
    pub Header: ACPI_PMTT_HEADER,
    pub ControllerId: u16,
    pub Reserved: u16,
}
pub type ACPI_PMTT_CONTROLLER = acpi_pmtt_controller;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pmtt_physical_component {
    pub Header: ACPI_PMTT_HEADER,
    pub BiosHandle: u32,
}
pub type ACPI_PMTT_PHYSICAL_COMPONENT = acpi_pmtt_physical_component;
#[repr(C)]
#[derive(Debug)]
pub struct acpi_pmtt_vendor_specific {
    pub Header: ACPI_PMTT_HEADER,
    pub TypeUuid: [u8; 16usize],
    Specific: __IncompleteArrayField<u8>,
}
pub type ACPI_PMTT_VENDOR_SPECIFIC = acpi_pmtt_vendor_specific;