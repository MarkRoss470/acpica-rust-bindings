use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};

///  CEDT - CXL Early Discovery Table
///         Version 1
/// 
///  Conforms to the \"CXL Early Discovery Table\" (CXL 2.0)
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_cedt {
    pub Header: FfiAcpiTableHeader,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_cedt_header {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiCedtType {
    Chbs = 0,
    Cfmws = 1,
    Reserved = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_cedt_chbs {
    pub Header: acpi_cedt_header,
    pub Uid: u32,
    pub CxlVersion: u32,
    pub Reserved: u32,
    pub Base: u64,
    pub Length: u64,
}

#[repr(C, packed)]
pub struct acpi_cedt_cfmws {
    pub Header: acpi_cedt_header,
    pub Reserved1: u32,
    pub BaseHpa: u64,
    pub WindowSize: u64,
    pub InterleaveWays: u8,
    pub InterleaveArithmetic: u8,
    pub Reserved2: u16,
    pub Granularity: u32,
    pub Restrictions: u16,
    pub QtgId: u16,
    InterleaveTargets: __IncompleteArrayField<u32>,
}