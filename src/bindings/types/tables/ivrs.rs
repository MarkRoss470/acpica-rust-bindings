use crate::bindings::types::FfiAcpiTableHeader;

///  IVRS - I/O Virtualization Reporting Structure
///         Version 1
/// 
///  Conforms to \"AMD I/O Virtualization Technology (IOMMU) Specification\",
///  Revision 1.26, February 2009.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_ivrs {
    pub Header: FfiAcpiTableHeader,
    pub Info: u32,
    pub Reserved: u64,
}

///  IVRS - I/O Virtualization Reporting Structure
///         Version 1
/// 
///  Conforms to \"AMD I/O Virtualization Technology (IOMMU) Specification\",
///  Revision 1.26, February 2009.
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_header {
    pub Type: u8,
    pub Flags: u8,
    pub Length: u16,
    pub DeviceId: u16,
}

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
    pub Header: acpi_ivrs_header,
    pub CapabilityOffset: u16,
    pub BaseAddress: u64,
    pub PciSegmentGroup: u16,
    pub Info: u16,
    pub FeatureReporting: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_hardware_11 {
    pub Header: acpi_ivrs_header,
    pub CapabilityOffset: u16,
    pub BaseAddress: u64,
    pub PciSegmentGroup: u16,
    pub Info: u16,
    pub Attributes: u32,
    pub EfrRegisterImage: u64,
    pub Reserved: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_de_header {
    pub Type: u8,
    pub Id: u16,
    pub DataSetting: u8,
}

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
    pub Header: acpi_ivrs_de_header,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device8a {
    pub Header: acpi_ivrs_de_header,
    pub Reserved1: u8,
    pub UsedId: u16,
    pub Reserved2: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device8b {
    pub Header: acpi_ivrs_de_header,
    pub ExtendedData: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device8c {
    pub Header: acpi_ivrs_de_header,
    pub Handle: u8,
    pub UsedId: u16,
    pub Variety: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_device_hid {
    pub Header: acpi_ivrs_de_header,
    pub AcpiHid: u64,
    pub AcpiCid: u64,
    pub UidType: u8,
    pub UidLength: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_ivrs_memory {
    pub Header: acpi_ivrs_de_header,
    pub AuxData: u16,
    pub Reserved: u64,
    pub StartAddress: u64,
    pub MemoryLength: u64,
}
