use crate::bindings::types::FfiAcpiTableHeader;

///  PHAT - Platform Health Assessment Table (ACPI 6.4)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePhat {
    pub Header: FfiAcpiTableHeader,
}
///  PHAT - Platform Health Assessment Table (ACPI 6.4)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPhatHeader {
    pub Type: u16,
    pub Length: u16,
    pub Revision: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPhatVersionData {
    pub Header: FfiAcpiPhatHeader,
    pub Reserved: [u8; 3usize],
    pub ElementCount: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPhatVersionElement {
    pub Guid: [u8; 16usize],
    pub VersionValue: u64,
    pub ProducerId: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPhatHealthData {
    pub Header: FfiAcpiPhatHeader,
    pub Reserved: [u8; 2usize],
    pub Health: u8,
    pub DeviceGuid: [u8; 16usize],
    pub DeviceSpecificOffset: u32,
}
