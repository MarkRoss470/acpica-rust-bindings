#[doc = " WAET - Windows ACPI Emulated devices Table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Windows ACPI Emulated Devices Table\", version 1.0, April 6, 2009"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_waet {
    pub Header: AcpiTableHeader,
    pub Flags: u32,
}
#[doc = " WAET - Windows ACPI Emulated devices Table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Windows ACPI Emulated Devices Table\", version 1.0, April 6, 2009"]
#[doc = ""]
pub type ACPI_TABLE_WAET = acpi_table_waet;

#[doc = " WPBT - Windows Platform Environment Table (ACPI 6.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Windows Platform Binary Table (WPBT)\" 29 November 2011"]
#[doc = ""]
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
#[doc = " WPBT - Windows Platform Environment Table (ACPI 6.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Windows Platform Binary Table (WPBT)\" 29 November 2011"]
#[doc = ""]
pub type ACPI_TABLE_WPBT = acpi_table_wpbt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_wpbt_unicode {
    pub UnicodeString: *mut u16,
}
pub type ACPI_WPBT_UNICODE = acpi_wpbt_unicode;
#[doc = " WSMT - Windows SMM Security Mitigations Table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Windows SMM Security Mitigations Table\","]
#[doc = " Version 1.0, April 18, 2016"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_wsmt {
    pub Header: AcpiTableHeader,
    pub ProtectionFlags: u32,
}
#[doc = " WSMT - Windows SMM Security Mitigations Table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Windows SMM Security Mitigations Table\","]
#[doc = " Version 1.0, April 18, 2016"]
#[doc = ""]
pub type ACPI_TABLE_WSMT = acpi_table_wsmt;