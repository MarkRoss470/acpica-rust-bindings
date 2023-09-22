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
    pub header: FfiAcpiTableHeader,
    pub width: u8,
    pub flags: u8,
    pub reserved: [u8; 10usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarHeader {
    pub header_type: u16,
    pub length: u16,
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
    pub entry_type: u8,
    pub length: u8,
    pub reserved: u16,
    pub enumeration_id: u8,
    pub bus: u8,
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
    pub device: u8,
    pub function: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarHardwareUnit {
    pub header: FfiAcpiDmarHeader,
    pub flags: u8,
    pub reserved: u8,
    pub segment: u16,
    pub address: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarReservedMemory {
    pub header: FfiAcpiDmarHeader,
    pub reserved: u16,
    pub segment: u16,
    pub base_address: u64,
    pub end_address: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarAtsr {
    pub header: FfiAcpiDmarHeader,
    pub flags: u8,
    pub reserved: u8,
    pub segment: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarRhsa {
    pub header: FfiAcpiDmarHeader,
    pub reserved: u32,
    pub base_address: u64,
    pub proximity_domain: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDmarAndd {
    pub header: FfiAcpiDmarHeader,
    pub reserved: [u8; 3usize],
    pub device_number: u8,
    pub device_name: [i8; 1usize],
}