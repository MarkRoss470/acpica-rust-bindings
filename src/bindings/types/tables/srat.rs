use crate::bindings::types::FfiAcpiTableHeader;

use super::FfiAcpiSubtableHeader;


///  SRAT - System Resource Affinity Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSrat {
    pub header: FfiAcpiTableHeader,
    pub table_revision: u32,
    pub reserved: u64,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiSratType {
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
pub struct FfiAcpiSratCpuAffinity {
    pub header: FfiAcpiSubtableHeader,
    pub proximity_domain_lo: u8,
    pub apic_id: u8,
    pub flags: u32,
    pub local_sapic_eid: u8,
    pub proximity_domain_hi: [u8; 3usize],
    pub clock_domain: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratMemAffinity {
    pub header: FfiAcpiSubtableHeader,
    pub proximity_domain: u32,
    pub reserved: u16,
    pub base_address: u64,
    pub length: u64,
    pub reserved1: u32,
    pub flags: u32,
    pub reserved2: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratX2apicCpuAffinity {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub proximity_domain: u32,
    pub apic_id: u32,
    pub flags: u32,
    pub clock_domain: u32,
    pub reserved2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratGiccAffinity {
    pub header: FfiAcpiSubtableHeader,
    pub proximity_domain: u32,
    pub acpi_processor_uid: u32,
    pub flags: u32,
    pub clock_domain: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratGicItsAffinity {
    pub header: FfiAcpiSubtableHeader,
    pub proximity_domain: u32,
    pub reserved: u16,
    pub its_id: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratGenericAffinity {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u8,
    pub device_handle_type: u8,
    pub proximity_domain: u32,
    pub device_handle: [u8; 16usize],
    pub flags: u32,
    pub reserved1: u32,
}
