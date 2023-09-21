use crate::types::AcpiTableHeader;

use super::ACPI_SUBTABLE_HEADER;

///  PPTT - Processor Properties Topology Table (ACPI 6.2)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_pptt {
    pub Header: AcpiTableHeader,
}
///  PPTT - Processor Properties Topology Table (ACPI 6.2)
///         Version 1
/// 
pub type ACPI_TABLE_PPTT = acpi_table_pptt;
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
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub Flags: u32,
    pub Parent: u32,
    pub AcpiProcessorId: u32,
    pub NumberOfPrivResources: u32,
}
pub type ACPI_PPTT_PROCESSOR = acpi_pptt_processor;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_cache {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub Flags: u32,
    pub NextLevelOfCache: u32,
    pub Size: u32,
    pub NumberOfSets: u32,
    pub Associativity: u8,
    pub Attributes: u8,
    pub LineSize: u16,
}
pub type ACPI_PPTT_CACHE = acpi_pptt_cache;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_cache_v1 {
    pub CacheId: u32,
}
pub type ACPI_PPTT_CACHE_V1 = acpi_pptt_cache_v1;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pptt_id {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub VendorId: u32,
    pub Level1Id: u64,
    pub Level2Id: u64,
    pub MajorRev: u16,
    pub MinorRev: u16,
    pub SpinRev: u16,
}
pub type ACPI_PPTT_ID = acpi_pptt_id;