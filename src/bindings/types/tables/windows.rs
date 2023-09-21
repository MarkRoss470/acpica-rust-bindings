use crate::types::AcpiTableHeader;

///  WAET - Windows ACPI Emulated devices Table
///         Version 1
/// 
///  Conforms to \"Windows ACPI Emulated Devices Table\", version 1.0, April 6, 2009
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_waet {
    pub Header: AcpiTableHeader,
    pub Flags: u32,
}
///  WAET - Windows ACPI Emulated devices Table
///         Version 1
/// 
///  Conforms to \"Windows ACPI Emulated Devices Table\", version 1.0, April 6, 2009
/// 
pub type ACPI_TABLE_WAET = acpi_table_waet;

///  WPBT - Windows Platform Environment Table (ACPI 6.0)
///         Version 1
/// 
///  Conforms to \"Windows Platform Binary Table (WPBT)\" 29 November 2011
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_wpbt {
    pub Header: AcpiTableHeader,
    pub HandoffSize: u32,
    pub HandoffAddress: u64,
    pub Layout: u8,
    pub Type: u8,
    pub ArgumentsLength: u16,
}
///  WPBT - Windows Platform Environment Table (ACPI 6.0)
///         Version 1
/// 
///  Conforms to \"Windows Platform Binary Table (WPBT)\" 29 November 2011
/// 
pub type ACPI_TABLE_WPBT = acpi_table_wpbt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_wpbt_unicode {
    pub UnicodeString: *mut u16,
}
pub type ACPI_WPBT_UNICODE = acpi_wpbt_unicode;
///  WSMT - Windows SMM Security Mitigations Table
///         Version 1
/// 
///  Conforms to \"Windows SMM Security Mitigations Table\",
///  Version 1.0, April 18, 2016
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_wsmt {
    pub Header: AcpiTableHeader,
    pub ProtectionFlags: u32,
}
///  WSMT - Windows SMM Security Mitigations Table
///         Version 1
/// 
///  Conforms to \"Windows SMM Security Mitigations Table\",
///  Version 1.0, April 18, 2016
/// 
pub type ACPI_TABLE_WSMT = acpi_table_wsmt;