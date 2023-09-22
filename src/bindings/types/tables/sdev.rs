use crate::bindings::types::FfiAcpiTableHeader;


///  SDEV - Secure Devices Table (ACPI 6.2)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSdev {
    pub Header: FfiAcpiTableHeader,
}
///  SDEV - Secure Devices Table (ACPI 6.2)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevHeader {
    pub Type: u8,
    pub Flags: u8,
    pub Length: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiSdevType {
    ACPI_SDEV_TYPE_NAMESPACE_DEVICE = 0,
    ACPI_SDEV_TYPE_PCIE_ENDPOINT_DEVICE = 1,
    ACPI_SDEV_TYPE_RESERVED = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevNamespace {
    pub Header: FfiAcpiSdevHeader,
    pub DeviceIdOffset: u16,
    pub DeviceIdLength: u16,
    pub VendorDataOffset: u16,
    pub VendorDataLength: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevSecureComponent {
    pub SecureComponentOffset: u16,
    pub SecureComponentLength: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevComponent {
    pub Header: FfiAcpiSdevHeader,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiSacType {
    ACPI_SDEV_TYPE_ID_COMPONENT = 0,
    ACPI_SDEV_TYPE_MEM_COMPONENT = 1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevIdComponent {
    pub Header: FfiAcpiSdevHeader,
    pub HardwareIdOffset: u16,
    pub HardwareIdLength: u16,
    pub SubsystemIdOffset: u16,
    pub SubsystemIdLength: u16,
    pub HardwareRevision: u16,
    pub HardwareRevPresent: u8,
    pub ClassCodePresent: u8,
    pub PciBaseClass: u8,
    pub PciSubClass: u8,
    pub PciProgrammingXface: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevMemComponent {
    pub Header: FfiAcpiSdevHeader,
    pub Reserved: u32,
    pub MemoryBaseAddress: u64,
    pub MemoryLength: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevPcie {
    pub Header: FfiAcpiSdevHeader,
    pub Segment: u16,
    pub StartBus: u16,
    pub PathOffset: u16,
    pub PathLength: u16,
    pub VendorDataOffset: u16,
    pub VendorDataLength: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSdevPciePath {
    pub Device: u8,
    pub Function: u8,
}
