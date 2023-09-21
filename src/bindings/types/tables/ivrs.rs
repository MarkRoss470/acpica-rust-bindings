use crate::types::AcpiTableHeader;

///  IVRS - I/O Virtualization Reporting Structure
///         Version 1
/// 
///  Conforms to \"AMD I/O Virtualization Technology (IOMMU) Specification\",
///  Revision 1.26, February 2009.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_ivrs {
    pub Header: AcpiTableHeader,
    pub Info: u32,
    pub Reserved: u64,
}

///  IVRS - I/O Virtualization Reporting Structure
///         Version 1
/// 
///  Conforms to \"AMD I/O Virtualization Technology (IOMMU) Specification\",
///  Revision 1.26, February 2009.
/// 
pub type ACPI_TABLE_IVRS = acpi_table_ivrs;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_header {
    pub Type: u8,
    pub Flags: u8,
    pub Length: u16,
    pub DeviceId: u16,
}
pub type ACPI_IVRS_HEADER = acpi_ivrs_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiIvrsType {
    ACPI_IVRS_TYPE_HARDWARE1 = 16,
    ACPI_IVRS_TYPE_HARDWARE2 = 17,
    ACPI_IVRS_TYPE_HARDWARE3 = 64,
    ACPI_IVRS_TYPE_MEMORY1 = 32,
    ACPI_IVRS_TYPE_MEMORY2 = 33,
    ACPI_IVRS_TYPE_MEMORY3 = 34,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_hardware_10 {
    pub Header: ACPI_IVRS_HEADER,
    pub CapabilityOffset: u16,
    pub BaseAddress: u64,
    pub PciSegmentGroup: u16,
    pub Info: u16,
    pub FeatureReporting: u32,
}
pub type ACPI_IVRS_HARDWARE1 = acpi_ivrs_hardware_10;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_hardware_11 {
    pub Header: ACPI_IVRS_HEADER,
    pub CapabilityOffset: u16,
    pub BaseAddress: u64,
    pub PciSegmentGroup: u16,
    pub Info: u16,
    pub Attributes: u32,
    pub EfrRegisterImage: u64,
    pub Reserved: u64,
}
pub type ACPI_IVRS_HARDWARE2 = acpi_ivrs_hardware_11;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_de_header {
    pub Type: u8,
    pub Id: u16,
    pub DataSetting: u8,
}
pub type ACPI_IVRS_DE_HEADER = acpi_ivrs_de_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiIvrsDeviceEntryType {
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
pub struct acpi_ivrs_device4 {
    pub Header: ACPI_IVRS_DE_HEADER,
}
pub type ACPI_IVRS_DEVICE4 = acpi_ivrs_device4;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device8a {
    pub Header: ACPI_IVRS_DE_HEADER,
    pub Reserved1: u8,
    pub UsedId: u16,
    pub Reserved2: u8,
}
pub type ACPI_IVRS_DEVICE8A = acpi_ivrs_device8a;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device8b {
    pub Header: ACPI_IVRS_DE_HEADER,
    pub ExtendedData: u32,
}
pub type ACPI_IVRS_DEVICE8B = acpi_ivrs_device8b;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device8c {
    pub Header: ACPI_IVRS_DE_HEADER,
    pub Handle: u8,
    pub UsedId: u16,
    pub Variety: u8,
}
pub type ACPI_IVRS_DEVICE8C = acpi_ivrs_device8c;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device_hid {
    pub Header: ACPI_IVRS_DE_HEADER,
    pub AcpiHid: u64,
    pub AcpiCid: u64,
    pub UidType: u8,
    pub UidLength: u8,
}
pub type ACPI_IVRS_DEVICE_HID = acpi_ivrs_device_hid;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_memory {
    pub Header: ACPI_IVRS_HEADER,
    pub AuxData: u16,
    pub Reserved: u64,
    pub StartAddress: u64,
    pub MemoryLength: u64,
}
pub type ACPI_IVRS_MEMORY = acpi_ivrs_memory;