use crate::bindings::types::FfiAcpiTableHeader;


///  SDEV - Secure Devices Table (ACPI 6.2)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_sdev {
    pub Header: FfiAcpiTableHeader,
}
///  SDEV - Secure Devices Table (ACPI 6.2)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_header {
    pub Type: u8,
    pub Flags: u8,
    pub Length: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiSdevType {
    ACPI_SDEV_TYPE_NAMESPACE_DEVICE = 0,
    ACPI_SDEV_TYPE_PCIE_ENDPOINT_DEVICE = 1,
    ACPI_SDEV_TYPE_RESERVED = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_namespace {
    pub Header: acpi_sdev_header,
    pub DeviceIdOffset: u16,
    pub DeviceIdLength: u16,
    pub VendorDataOffset: u16,
    pub VendorDataLength: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_secure_component {
    pub SecureComponentOffset: u16,
    pub SecureComponentLength: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_component {
    pub Header: acpi_sdev_header,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiSacType {
    ACPI_SDEV_TYPE_ID_COMPONENT = 0,
    ACPI_SDEV_TYPE_MEM_COMPONENT = 1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_id_component {
    pub Header: acpi_sdev_header,
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
pub struct acpi_sdev_mem_component {
    pub Header: acpi_sdev_header,
    pub Reserved: u32,
    pub MemoryBaseAddress: u64,
    pub MemoryLength: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_pcie {
    pub Header: acpi_sdev_header,
    pub Segment: u16,
    pub StartBus: u16,
    pub PathOffset: u16,
    pub PathLength: u16,
    pub VendorDataOffset: u16,
    pub VendorDataLength: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_sdev_pcie_path {
    pub Device: u8,
    pub Function: u8,
}
