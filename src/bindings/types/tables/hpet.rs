#[doc = " HPET - High Precision Event Timer table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"IA-PC HPET (High Precision Event Timers) Specification\","]
#[doc = " Version 1.0a, October 2004"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_hpet {
    pub Header: AcpiTableHeader,
    pub Id: u32,
    pub Address: ACPI_GENERIC_ADDRESS,
    pub Sequence: u8,
    pub MinimumTick: u16,
    pub Flags: u8,
}
#[doc = " HPET - High Precision Event Timer table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"IA-PC HPET (High Precision Event Timers) Specification\","]
#[doc = " Version 1.0a, October 2004"]
#[doc = ""]
pub type ACPI_TABLE_HPET = acpi_table_hpet;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiHpetPageProtect {
    ACPI_HPET_NO_PAGE_PROTECT = 0,
    ACPI_HPET_PAGE_PROTECT4 = 1,
    ACPI_HPET_PAGE_PROTECT64 = 2,
}