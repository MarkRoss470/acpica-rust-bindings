use crate::types::AcpiTableHeader;

#[doc = " HMAT - Heterogeneous Memory Attributes Table (ACPI 6.3)"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_hmat {
    pub Header: AcpiTableHeader,
    pub Reserved: u32,
}
#[doc = " HMAT - Heterogeneous Memory Attributes Table (ACPI 6.3)"]
#[doc = ""]
pub type ACPI_TABLE_HMAT = acpi_table_hmat;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiHmatType {
    ACPI_HMAT_TYPE_ADDRESS_RANGE = 0,
    ACPI_HMAT_TYPE_LOCALITY = 1,
    ACPI_HMAT_TYPE_CACHE = 2,
    ACPI_HMAT_TYPE_RESERVED = 3,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hmat_structure {
    pub Type: u16,
    pub Reserved: u16,
    pub Length: u32,
}
pub type ACPI_HMAT_STRUCTURE = acpi_hmat_structure;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hmat_proximity_domain {
    pub Header: ACPI_HMAT_STRUCTURE,
    pub Flags: u16,
    pub Reserved1: u16,
    pub InitiatorPD: u32,
    pub MemoryPD: u32,
    pub Reserved2: u32,
    pub Reserved3: u64,
    pub Reserved4: u64,
}
pub type ACPI_HMAT_PROXIMITY_DOMAIN = acpi_hmat_proximity_domain;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hmat_locality {
    pub Header: ACPI_HMAT_STRUCTURE,
    pub Flags: u8,
    pub DataType: u8,
    pub MinTransferSize: u8,
    pub Reserved1: u8,
    pub NumberOfInitiatorPDs: u32,
    pub NumberOfTargetPDs: u32,
    pub Reserved2: u32,
    pub EntryBaseUnit: u64,
}
pub type ACPI_HMAT_LOCALITY = acpi_hmat_locality;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hmat_cache {
    pub Header: ACPI_HMAT_STRUCTURE,
    pub MemoryPD: u32,
    pub Reserved1: u32,
    pub CacheSize: u64,
    pub CacheAttributes: u32,
    pub Reserved2: u16,
    pub NumberOfSMBIOSHandles: u16,
}
pub type ACPI_HMAT_CACHE = acpi_hmat_cache;