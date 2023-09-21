#[doc = " RSDT/XSDT - Root System Description Tables"]
#[doc = "             Version 1 (both)"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_rsdt {
    pub Header: AcpiTableHeader,
    pub TableOffsetEntry: [u32; 1usize],
}
#[doc = " RSDT/XSDT - Root System Description Tables"]
#[doc = "             Version 1 (both)"]
#[doc = ""]
pub type ACPI_TABLE_RSDT = acpi_table_rsdt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_xsdt {
    pub Header: AcpiTableHeader,
    pub TableOffsetEntry: [u64; 1usize],
}
pub type ACPI_TABLE_XSDT = acpi_table_xsdt;