use crate::bindings::types::FfiAcpiTableHeader;

///  RSDT/XSDT - Root System Description Tables
///              Version 1 (both)
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableRsdt {
    pub Header: FfiAcpiTableHeader,
    pub TableOffsetEntry: [u32; 1usize],
}
///  RSDT/XSDT - Root System Description Tables
///              Version 1 (both)
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableXsdt {
    pub Header: FfiAcpiTableHeader,
    pub TableOffsetEntry: [u64; 1usize],
}
