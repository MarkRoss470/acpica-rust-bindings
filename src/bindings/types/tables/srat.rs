use crate::bindings::types::FfiAcpiTableHeader;

use super::acpi_subtable_header;


///  SRAT - System Resource Affinity Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_srat {
    pub Header: FfiAcpiTableHeader,
    pub TableRevision: u32,
    pub Reserved: u64,
}
///  SRAT - System Resource Affinity Table
///         Version 3
/// 

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiSratType {
    ACPI_SRAT_TYPE_CPU_AFFINITY = 0,
    ACPI_SRAT_TYPE_MEMORY_AFFINITY = 1,
    ACPI_SRAT_TYPE_X2APIC_CPU_AFFINITY = 2,
    ACPI_SRAT_TYPE_GICC_AFFINITY = 3,
    ACPI_SRAT_TYPE_GIC_ITS_AFFINITY = 4,
    ACPI_SRAT_TYPE_GENERIC_AFFINITY = 5,
    ACPI_SRAT_TYPE_RESERVED = 6,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_cpu_affinity {
    pub Header: acpi_subtable_header,
    pub ProximityDomainLo: u8,
    pub ApicId: u8,
    pub Flags: u32,
    pub LocalSapicEid: u8,
    pub ProximityDomainHi: [u8; 3usize],
    pub ClockDomain: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_mem_affinity {
    pub Header: acpi_subtable_header,
    pub ProximityDomain: u32,
    pub Reserved: u16,
    pub BaseAddress: u64,
    pub Length: u64,
    pub Reserved1: u32,
    pub Flags: u32,
    pub Reserved2: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_x2apic_cpu_affinity {
    pub Header: acpi_subtable_header,
    pub Reserved: u16,
    pub ProximityDomain: u32,
    pub ApicId: u32,
    pub Flags: u32,
    pub ClockDomain: u32,
    pub Reserved2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_gicc_affinity {
    pub Header: acpi_subtable_header,
    pub ProximityDomain: u32,
    pub AcpiProcessorUid: u32,
    pub Flags: u32,
    pub ClockDomain: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_gic_its_affinity {
    pub Header: acpi_subtable_header,
    pub ProximityDomain: u32,
    pub Reserved: u16,
    pub ItsId: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_generic_affinity {
    pub Header: acpi_subtable_header,
    pub Reserved: u8,
    pub DeviceHandleType: u8,
    pub ProximityDomain: u32,
    pub DeviceHandle: [u8; 16usize],
    pub Flags: u32,
    pub Reserved1: u32,
}
