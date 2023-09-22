use crate::bindings::types::FfiAcpiTableHeader;

use super::acpi_subtable_header;

///  PPTT - Processor Properties Topology Table (ACPI 6.2)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_pptt {
    pub Header: FfiAcpiTableHeader,
}
///  PPTT - Processor Properties Topology Table (ACPI 6.2)
///         Version 1
/// 

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiPpttType {
    ACPI_PPTT_TYPE_PROCESSOR = 0,
    ACPI_PPTT_TYPE_CACHE = 1,
    ACPI_PPTT_TYPE_ID = 2,
    ACPI_PPTT_TYPE_RESERVED = 3,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_processor {
    pub Header: acpi_subtable_header,
    pub Reserved: u16,
    pub Flags: u32,
    pub Parent: u32,
    pub AcpiProcessorId: u32,
    pub NumberOfPrivResources: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_cache {
    pub Header: acpi_subtable_header,
    pub Reserved: u16,
    pub Flags: u32,
    pub NextLevelOfCache: u32,
    pub Size: u32,
    pub NumberOfSets: u32,
    pub Associativity: u8,
    pub Attributes: u8,
    pub LineSize: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_cache_v1 {
    pub CacheId: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_id {
    pub Header: acpi_subtable_header,
    pub Reserved: u16,
    pub VendorId: u32,
    pub Level1Id: u64,
    pub Level2Id: u64,
    pub MajorRev: u16,
    pub MinorRev: u16,
    pub SpinRev: u16,
}
