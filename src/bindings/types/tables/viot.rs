use crate::bindings::types::FfiAcpiTableHeader;

///  VIOT - Virtual I/O Translation Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableViot {
    pub header: FfiAcpiTableHeader,
    pub node_count: u16,
    pub node_offset: u16,
    pub reserved: [u8; 8usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiViotHeader {
    pub header_type: u8,
    pub reserved: u8,
    pub length: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(dead_code)] // FFI type so variants are not explicitly constructed
pub enum FfiAcpiViotNodeType {
    PciRange = 1,
    Mmio = 2,
    VirtioIommuPci = 3,
    VirtioIommuMmio = 4,
    Reserved = 5,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiViotPciRange {
    pub header: FfiAcpiViotHeader,
    pub endpoint_start: u32,
    pub segment_start: u16,
    pub segment_end: u16,
    pub bdf_start: u16,
    pub bdf_end: u16,
    pub output_node: u16,
    pub reserved: [u8; 6usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiViotMmio {
    pub header: FfiAcpiViotHeader,
    pub endpoint: u32,
    pub base_address: u64,
    pub output_node: u16,
    pub reserved: [u8; 6usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiViotVirtioIommuPci {
    pub header: FfiAcpiViotHeader,
    pub segment: u16,
    pub bdf: u16,
    pub reserved: [u8; 8usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiViotVirtioIommuMmio {
    pub header: FfiAcpiViotHeader,
    pub reserved: [u8; 4usize],
    pub base_address: u64,
}
