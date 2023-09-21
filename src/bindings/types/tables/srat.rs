use crate::types::AcpiTableHeader;

use super::ACPI_SUBTABLE_HEADER;


///  SRAT - System Resource Affinity Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_srat {
    pub Header: AcpiTableHeader,
    pub TableRevision: u32,
    pub Reserved: u64,
}
///  SRAT - System Resource Affinity Table
///         Version 3
/// 
pub type ACPI_TABLE_SRAT = acpi_table_srat;
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
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProximityDomainLo: u8,
    pub ApicId: u8,
    pub Flags: u32,
    pub LocalSapicEid: u8,
    pub ProximityDomainHi: [u8; 3usize],
    pub ClockDomain: u32,
}
pub type ACPI_SRAT_CPU_AFFINITY = acpi_srat_cpu_affinity;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_mem_affinity {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProximityDomain: u32,
    pub Reserved: u16,
    pub BaseAddress: u64,
    pub Length: u64,
    pub Reserved1: u32,
    pub Flags: u32,
    pub Reserved2: u64,
}
pub type ACPI_SRAT_MEM_AFFINITY = acpi_srat_mem_affinity;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_x2apic_cpu_affinity {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub ProximityDomain: u32,
    pub ApicId: u32,
    pub Flags: u32,
    pub ClockDomain: u32,
    pub Reserved2: u32,
}
pub type ACPI_SRAT_X2APIC_CPU_AFFINITY = acpi_srat_x2apic_cpu_affinity;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_gicc_affinity {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProximityDomain: u32,
    pub AcpiProcessorUid: u32,
    pub Flags: u32,
    pub ClockDomain: u32,
}
pub type ACPI_SRAT_GICC_AFFINITY = acpi_srat_gicc_affinity;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_gic_its_affinity {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProximityDomain: u32,
    pub Reserved: u16,
    pub ItsId: u32,
}
pub type ACPI_SRAT_GIC_ITS_AFFINITY = acpi_srat_gic_its_affinity;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_srat_generic_affinity {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u8,
    pub DeviceHandleType: u8,
    pub ProximityDomain: u32,
    pub DeviceHandle: [u8; 16usize],
    pub Flags: u32,
    pub Reserved1: u32,
}
pub type ACPI_SRAT_GENERIC_AFFINITY = acpi_srat_generic_affinity;