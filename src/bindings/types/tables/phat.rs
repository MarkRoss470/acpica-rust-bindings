#[doc = " PHAT - Platform Health Assessment Table (ACPI 6.4)"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_phat {
    pub Header: AcpiTableHeader,
}
#[doc = " PHAT - Platform Health Assessment Table (ACPI 6.4)"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_PHAT = acpi_table_phat;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_phat_header {
    pub Type: u16,
    pub Length: u16,
    pub Revision: u8,
}
pub type ACPI_PHAT_HEADER = acpi_phat_header;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_phat_version_data {
    pub Header: ACPI_PHAT_HEADER,
    pub Reserved: [u8; 3usize],
    pub ElementCount: u32,
}
pub type ACPI_PHAT_VERSION_DATA = acpi_phat_version_data;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_phat_version_element {
    pub Guid: [u8; 16usize],
    pub VersionValue: u64,
    pub ProducerId: u32,
}
pub type ACPI_PHAT_VERSION_ELEMENT = acpi_phat_version_element;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_phat_health_data {
    pub Header: ACPI_PHAT_HEADER,
    pub Reserved: [u8; 2usize],
    pub Health: u8,
    pub DeviceGuid: [u8; 16usize],
    pub DeviceSpecificOffset: u32,
}
pub type ACPI_PHAT_HEALTH_DATA = acpi_phat_health_data;