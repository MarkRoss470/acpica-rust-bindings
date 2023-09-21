use crate::{types::AcpiTableHeader, bindings::types::__IncompleteArrayField};


///  CSRT - Core System Resource Table
///         Version 0
/// 
///  Conforms to the \"Core System Resource Table (CSRT)\", November 14, 2011
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_csrt {
    pub Header: AcpiTableHeader,
}
///  CSRT - Core System Resource Table
///         Version 0
/// 
///  Conforms to the \"Core System Resource Table (CSRT)\", November 14, 2011
/// 
pub type ACPI_TABLE_CSRT = acpi_table_csrt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_csrt_group {
    pub Length: u32,
    pub VendorId: u32,
    pub SubvendorId: u32,
    pub DeviceId: u16,
    pub SubdeviceId: u16,
    pub Revision: u16,
    pub Reserved: u16,
    pub SharedInfoLength: u32,
}
pub type ACPI_CSRT_GROUP = acpi_csrt_group;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_csrt_shared_info {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub MmioBaseLow: u32,
    pub MmioBaseHigh: u32,
    pub GsiInterrupt: u32,
    pub InterruptPolarity: u8,
    pub InterruptMode: u8,
    pub NumChannels: u8,
    pub DmaAddressWidth: u8,
    pub BaseRequestLine: u16,
    pub NumHandshakeSignals: u16,
    pub MaxBlockSize: u32,
}
pub type ACPI_CSRT_SHARED_INFO = acpi_csrt_shared_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_csrt_descriptor {
    pub Length: u32,
    pub Type: u16,
    pub Subtype: u16,
    pub Uid: u32,
}
pub type ACPI_CSRT_DESCRIPTOR = acpi_csrt_descriptor;