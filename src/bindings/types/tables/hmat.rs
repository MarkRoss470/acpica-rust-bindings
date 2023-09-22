use crate::bindings::types::FfiAcpiTableHeader;

///  HMAT - Heterogeneous Memory Attributes Table (ACPI 6.3)
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableHmat {
    pub Header: FfiAcpiTableHeader,
    pub Reserved: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiHmatType {
    ACPI_HMAT_TYPE_ADDRESS_RANGE = 0,
    ACPI_HMAT_TYPE_LOCALITY = 1,
    ACPI_HMAT_TYPE_CACHE = 2,
    ACPI_HMAT_TYPE_RESERVED = 3,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHmatStructure {
    pub Type: u16,
    pub Reserved: u16,
    pub Length: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHmatProximityDomain {
    pub Header: FfiAcpiHmatStructure,
    pub Flags: u16,
    pub Reserved1: u16,
    pub InitiatorPD: u32,
    pub MemoryPD: u32,
    pub Reserved2: u32,
    pub Reserved3: u64,
    pub Reserved4: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHmatLocality {
    pub Header: FfiAcpiHmatStructure,
    pub Flags: u8,
    pub DataType: u8,
    pub MinTransferSize: u8,
    pub Reserved1: u8,
    pub NumberOfInitiatorPDs: u32,
    pub NumberOfTargetPDs: u32,
    pub Reserved2: u32,
    pub EntryBaseUnit: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHmatCache {
    pub Header: FfiAcpiHmatStructure,
    pub MemoryPD: u32,
    pub Reserved1: u32,
    pub CacheSize: u64,
    pub CacheAttributes: u32,
    pub Reserved2: u16,
    pub NumberOfSMBIOSHandles: u16,
}