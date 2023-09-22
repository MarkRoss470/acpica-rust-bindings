use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  DMAR - DMA Remapping table
///         Version 1
/// 
///  Conforms to \"Intel Virtualization Technology for Directed I/O\",
///  Version 2.3, October 2014
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableDmar {
    pub Header: FfiAcpiTableHeader,
    pub Width: u8,
    pub Flags: u8,
    pub Reserved: [u8; 10usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarHeader {
    pub Type: u16,
    pub Length: u16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiDmarType {
    ACPI_DMAR_TYPE_HARDWARE_UNIT = 0,
    ACPI_DMAR_TYPE_RESERVED_MEMORY = 1,
    ACPI_DMAR_TYPE_ROOT_ATS = 2,
    ACPI_DMAR_TYPE_HARDWARE_AFFINITY = 3,
    ACPI_DMAR_TYPE_NAMESPACE = 4,
    ACPI_DMAR_TYPE_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarDeviceScope {
    pub EntryType: u8,
    pub Length: u8,
    pub Reserved: u16,
    pub EnumerationId: u8,
    pub Bus: u8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiDmarScopeType {
    ACPI_DMAR_SCOPE_TYPE_NOT_USED = 0,
    ACPI_DMAR_SCOPE_TYPE_ENDPOINT = 1,
    ACPI_DMAR_SCOPE_TYPE_BRIDGE = 2,
    ACPI_DMAR_SCOPE_TYPE_IOAPIC = 3,
    ACPI_DMAR_SCOPE_TYPE_HPET = 4,
    ACPI_DMAR_SCOPE_TYPE_NAMESPACE = 5,
    ACPI_DMAR_SCOPE_TYPE_RESERVED = 6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarPciPath {
    pub Device: u8,
    pub Function: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarHardwareUnit {
    pub Header: FfiAcpiDmarHeader,
    pub Flags: u8,
    pub Reserved: u8,
    pub Segment: u16,
    pub Address: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarReservedMemory {
    pub Header: FfiAcpiDmarHeader,
    pub Reserved: u16,
    pub Segment: u16,
    pub BaseAddress: u64,
    pub EndAddress: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarAtsr {
    pub Header: FfiAcpiDmarHeader,
    pub Flags: u8,
    pub Reserved: u8,
    pub Segment: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarRhsa {
    pub Header: FfiAcpiDmarHeader,
    pub Reserved: u32,
    pub BaseAddress: u64,
    pub ProximityDomain: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarAndd {
    pub Header: FfiAcpiDmarHeader,
    pub Reserved: [u8; 3usize],
    pub DeviceNumber: u8,
    pub DeviceName: [i8; 1usize],
}