use crate::{types::AcpiTableHeader, bindings::types::__IncompleteArrayField};


///  DRTM - Dynamic Root of Trust for Measurement table
///  Conforms to \"TCG D-RTM Architecture\" June 17 2013, Version 1.0.0
///  Table version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_drtm {
    pub Header: AcpiTableHeader,
    pub EntryBaseAddress: u64,
    pub EntryLength: u64,
    pub EntryAddress32: u32,
    pub EntryAddress64: u64,
    pub ExitAddress: u64,
    pub LogAreaAddress: u64,
    pub LogAreaLength: u32,
    pub ArchDependentAddress: u64,
    pub Flags: u32,
}
///  DRTM - Dynamic Root of Trust for Measurement table
///  Conforms to \"TCG D-RTM Architecture\" June 17 2013, Version 1.0.0
///  Table version 1
/// 
pub type ACPI_TABLE_DRTM = acpi_table_drtm;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_drtm_vtable_list {
    pub ValidatedTableCount: u32,
    pub ValidatedTables: [u64; 1usize],
}
pub type ACPI_DRTM_VTABLE_LIST = acpi_drtm_vtable_list;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_drtm_resource {
    pub Size: [u8; 7usize],
    pub Type: u8,
    pub Address: u64,
}
pub type ACPI_DRTM_RESOURCE = acpi_drtm_resource;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_drtm_resource_list {
    pub ResourceCount: u32,
    pub Resources: [ACPI_DRTM_RESOURCE; 1usize],
}
pub type ACPI_DRTM_RESOURCE_LIST = acpi_drtm_resource_list;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_drtm_dps_id {
    pub DpsIdLength: u32,
    pub DpsId: [u8; 16usize],
}
pub type ACPI_DRTM_DPS_ID = acpi_drtm_dps_id;