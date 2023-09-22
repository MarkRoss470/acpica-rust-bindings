use crate::bindings::types::FfiAcpiTableHeader;

///  IVRS - I/O Virtualization Reporting Structure
///         Version 1
/// 
///  Conforms to \"AMD I/O Virtualization Technology (IOMMU) Specification\",
///  Revision 1.26, February 2009.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableIvrs {
    pub header: FfiAcpiTableHeader,
    pub info: u32,
    pub reserved: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsHeader {
    pub header_type: u8,
    pub flags: u8,
    pub length: u16,
    pub device_id: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiIvrsType {
    ACPI_IVRS_TYPE_HARDWARE1 = 16,
    ACPI_IVRS_TYPE_HARDWARE2 = 17,
    ACPI_IVRS_TYPE_HARDWARE3 = 64,
    ACPI_IVRS_TYPE_MEMORY1 = 32,
    ACPI_IVRS_TYPE_MEMORY2 = 33,
    ACPI_IVRS_TYPE_MEMORY3 = 34,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsHardware10 {
    pub header: FfiAcpiIvrsHeader,
    pub capability_offset: u16,
    pub base_address: u64,
    pub pci_segment_group: u16,
    pub info: u16,
    pub feature_reporting: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsHardware11 {
    pub header: FfiAcpiIvrsHeader,
    pub capability_offset: u16,
    pub base_address: u64,
    pub pci_segment_group: u16,
    pub info: u16,
    pub attributes: u32,
    pub efr_register_image: u64,
    pub reserved: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsDeHeader {
    pub header_type: u8,
    pub id: u16,
    pub data_setting: u8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiIvrsDeviceEntryType {
    ACPI_IVRS_TYPE_PAD4 = 0,
    ACPI_IVRS_TYPE_ALL = 1,
    ACPI_IVRS_TYPE_SELECT = 2,
    ACPI_IVRS_TYPE_START = 3,
    ACPI_IVRS_TYPE_END = 4,
    ACPI_IVRS_TYPE_PAD8 = 64,
    ACPI_IVRS_TYPE_NOT_USED = 65,
    ACPI_IVRS_TYPE_ALIAS_SELECT = 66,
    ACPI_IVRS_TYPE_ALIAS_START = 67,
    ACPI_IVRS_TYPE_EXT_SELECT = 70,
    ACPI_IVRS_TYPE_EXT_START = 71,
    ACPI_IVRS_TYPE_SPECIAL = 72,
    ACPI_IVRS_TYPE_HID = 240,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsDevice4 {
    pub header: FfiAcpiIvrsDeHeader,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsDevice8a {
    pub header: FfiAcpiIvrsDeHeader,
    pub reserved1: u8,
    pub used_id: u16,
    pub reserved2: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsDevice8b {
    pub header: FfiAcpiIvrsDeHeader,
    pub extended_data: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsDevice8c {
    pub header: FfiAcpiIvrsDeHeader,
    pub handle: u8,
    pub used_id: u16,
    pub variety: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsDeviceHid {
    pub header: FfiAcpiIvrsDeHeader,
    pub acpi_hid: u64,
    pub acpi_cid: u64,
    pub uid_type: u8,
    pub uid_length: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIvrsMemory {
    pub header: FfiAcpiIvrsDeHeader,
    pub aux_data: u16,
    pub reserved: u64,
    pub start_address: u64,
    pub memory_length: u64,
}
