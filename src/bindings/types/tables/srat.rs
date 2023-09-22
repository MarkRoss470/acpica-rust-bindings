use crate::bindings::types::FfiAcpiTableHeader;

use super::FfiAcpiSubtableHeader;


///  SRAT - System Resource Affinity Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSrat {
    pub Header: FfiAcpiTableHeader,
    pub TableRevision: u32,
    pub Reserved: u64,
}
///  SRAT - System Resource Affinity Table
///         Version 3
/// 

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
    pub Header: FfiAcpiSubtableHeader,
    pub ProximityDomainLo: u8,
    pub ApicId: u8,
    pub Flags: u32,
    pub LocalSapicEid: u8,
    pub ProximityDomainHi: [u8; 3usize],
    pub ClockDomain: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratMemAffinity {
    pub Header: FfiAcpiSubtableHeader,
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
pub struct FfiAcpiSratX2apicCpuAffinity {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub ProximityDomain: u32,
    pub ApicId: u32,
    pub Flags: u32,
    pub ClockDomain: u32,
    pub Reserved2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratGiccAffinity {
    pub Header: FfiAcpiSubtableHeader,
    pub ProximityDomain: u32,
    pub AcpiProcessorUid: u32,
    pub Flags: u32,
    pub ClockDomain: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratGicItsAffinity {
    pub Header: FfiAcpiSubtableHeader,
    pub ProximityDomain: u32,
    pub Reserved: u16,
    pub ItsId: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSratGenericAffinity {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u8,
    pub DeviceHandleType: u8,
    pub ProximityDomain: u32,
    pub DeviceHandle: [u8; 16usize],
    pub Flags: u32,
    pub Reserved1: u32,
}
