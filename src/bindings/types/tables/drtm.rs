use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  DRTM - Dynamic Root of Trust for Measurement table
///  Conforms to \"TCG D-RTM Architecture\" June 17 2013, Version 1.0.0
///  Table version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableDrtm {
    pub Header: FfiAcpiTableHeader,
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

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDrtmVtableList {
    pub ValidatedTableCount: u32,
    pub ValidatedTables: [u64; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDrtmResource {
    pub Size: [u8; 7usize],
    pub Type: u8,
    pub Address: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDrtmResourceList {
    pub ResourceCount: u32,
    pub Resources: [FfiAcpiDrtmResource; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDrtmDpsId {
    pub DpsIdLength: u32,
    pub DpsId: [u8; 16usize],
}