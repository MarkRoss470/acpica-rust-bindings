#[doc = " DBG2 - Debug Port Table 2"]
#[doc = "        Version 0 (Both main table and subtables)"]
#[doc = ""]
#[doc = " Conforms to \"Microsoft Debug Port Table 2 (DBG2)\", September 21, 2020"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_dbg2 {
    pub Header: AcpiTableHeader,
    pub InfoOffset: u32,
    pub InfoCount: u32,
}
#[doc = " DBG2 - Debug Port Table 2"]
#[doc = "        Version 0 (Both main table and subtables)"]
#[doc = ""]
#[doc = " Conforms to \"Microsoft Debug Port Table 2 (DBG2)\", September 21, 2020"]
#[doc = ""]
pub type ACPI_TABLE_DBG2 = acpi_table_dbg2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dbg2_header {
    pub InfoOffset: u32,
    pub InfoCount: u32,
}
pub type ACPI_DBG2_HEADER = acpi_dbg2_header;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dbg2_device {
    pub Revision: u8,
    pub Length: u16,
    pub RegisterCount: u8,
    pub NamepathLength: u16,
    pub NamepathOffset: u16,
    pub OemDataLength: u16,
    pub OemDataOffset: u16,
    pub PortType: u16,
    pub PortSubtype: u16,
    pub Reserved: u16,
    pub BaseAddressOffset: u16,
    pub AddressSizeOffset: u16,
}
pub type ACPI_DBG2_DEVICE = acpi_dbg2_device;