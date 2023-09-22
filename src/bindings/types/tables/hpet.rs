use crate::{bindings::types::FfiAcpiTableHeader, interface::AcpiGenericAddress};

///  HPET - High Precision Event Timer table
///         Version 1
/// 
///  Conforms to \"IA-PC HPET (High Precision Event Timers) Specification\",
///  Version 1.0a, October 2004
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableHpet {
    pub header: FfiAcpiTableHeader,
    pub id: u32,
    pub address: AcpiGenericAddress,
    pub sequence: u8,
    pub minimum_tick: u16,
    pub flags: u8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiHpetPageProtect {
    ACPI_HPET_NO_PAGE_PROTECT = 0,
    ACPI_HPET_PAGE_PROTECT4 = 1,
    ACPI_HPET_PAGE_PROTECT64 = 2,
}