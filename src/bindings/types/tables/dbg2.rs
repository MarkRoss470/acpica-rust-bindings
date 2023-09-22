use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  DBG2 - Debug Port Table 2
///         Version 0 (Both main table and subtables)
/// 
///  Conforms to \"Microsoft Debug Port Table 2 (DBG2)\", September 21, 2020
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableDbg2 {
    pub Header: FfiAcpiTableHeader,
    pub InfoOffset: u32,
    pub InfoCount: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDbg2Header {
    pub InfoOffset: u32,
    pub InfoCount: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiDbg2Device {
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