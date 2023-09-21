use crate::{types::AcpiTableHeader, bindings::types::__IncompleteArrayField};

///  CEDT - CXL Early Discovery Table
///         Version 1
/// 
///  Conforms to the \"CXL Early Discovery Table\" (CXL 2.0)
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_cedt {
    pub Header: AcpiTableHeader,
}
///  CEDT - CXL Early Discovery Table
///         Version 1
/// 
///  Conforms to the \"CXL Early Discovery Table\" (CXL 2.0)
/// 
pub type ACPI_TABLE_CEDT = acpi_table_cedt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_cedt_header {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}
pub type ACPI_CEDT_HEADER = acpi_cedt_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiCedtType {
    ACPI_CEDT_TYPE_CHBS = 0,
    ACPI_CEDT_TYPE_CFMWS = 1,
    ACPI_CEDT_TYPE_RESERVED = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_cedt_chbs {
    pub Header: ACPI_CEDT_HEADER,
    pub Uid: u32,
    pub CxlVersion: u32,
    pub Reserved: u32,
    pub Base: u64,
    pub Length: u64,
}
pub type ACPI_CEDT_CHBS = acpi_cedt_chbs;
#[repr(C, packed)]
pub struct acpi_cedt_cfmws {
    pub Header: ACPI_CEDT_HEADER,
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
pub type ACPI_CEDT_CFMWS = acpi_cedt_cfmws;