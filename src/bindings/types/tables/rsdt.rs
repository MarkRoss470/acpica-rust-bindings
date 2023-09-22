use crate::bindings::types::FfiAcpiTableHeader;

///  RSDT/XSDT - Root System Description Tables
///              Version 1 (both)
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_rsdt {
    pub Header: FfiAcpiTableHeader,
    pub TableOffsetEntry: [u32; 1usize],
}
///  RSDT/XSDT - Root System Description Tables
///              Version 1 (both)
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_xsdt {
    pub Header: FfiAcpiTableHeader,
    pub TableOffsetEntry: [u64; 1usize],
}
