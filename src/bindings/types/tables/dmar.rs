#[doc = " DMAR - DMA Remapping table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Intel Virtualization Technology for Directed I/O\","]
#[doc = " Version 2.3, October 2014"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_dmar {
    pub Header: AcpiTableHeader,
    pub Width: u8,
    pub Flags: u8,
    pub Reserved: [u8; 10usize],
}
#[doc = " DMAR - DMA Remapping table"]
#[doc = "        Version 1"]
#[doc = ""]
#[doc = " Conforms to \"Intel Virtualization Technology for Directed I/O\","]
#[doc = " Version 2.3, October 2014"]
#[doc = ""]
pub type ACPI_TABLE_DMAR = acpi_table_dmar;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_header {
    pub Type: u16,
    pub Length: u16,
}
pub type ACPI_DMAR_HEADER = acpi_dmar_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiDmarType {
    ACPI_DMAR_TYPE_HARDWARE_UNIT = 0,
    ACPI_DMAR_TYPE_RESERVED_MEMORY = 1,
    ACPI_DMAR_TYPE_ROOT_ATS = 2,
    ACPI_DMAR_TYPE_HARDWARE_AFFINITY = 3,
    ACPI_DMAR_TYPE_NAMESPACE = 4,
    ACPI_DMAR_TYPE_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_device_scope {
    pub EntryType: u8,
    pub Length: u8,
    pub Reserved: u16,
    pub EnumerationId: u8,
    pub Bus: u8,
}
pub type ACPI_DMAR_DEVICE_SCOPE = acpi_dmar_device_scope;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiDmarScopeType {
    ACPI_DMAR_SCOPE_TYPE_NOT_USED = 0,
    ACPI_DMAR_SCOPE_TYPE_ENDPOINT = 1,
    ACPI_DMAR_SCOPE_TYPE_BRIDGE = 2,
    ACPI_DMAR_SCOPE_TYPE_IOAPIC = 3,
    ACPI_DMAR_SCOPE_TYPE_HPET = 4,
    ACPI_DMAR_SCOPE_TYPE_NAMESPACE = 5,
    ACPI_DMAR_SCOPE_TYPE_RESERVED = 6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_pci_path {
    pub Device: u8,
    pub Function: u8,
}
pub type ACPI_DMAR_PCI_PATH = acpi_dmar_pci_path;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_hardware_unit {
    pub Header: ACPI_DMAR_HEADER,
    pub Flags: u8,
    pub Reserved: u8,
    pub Segment: u16,
    pub Address: u64,
}
pub type ACPI_DMAR_HARDWARE_UNIT = acpi_dmar_hardware_unit;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_reserved_memory {
    pub Header: ACPI_DMAR_HEADER,
    pub Reserved: u16,
    pub Segment: u16,
    pub BaseAddress: u64,
    pub EndAddress: u64,
}
pub type ACPI_DMAR_RESERVED_MEMORY = acpi_dmar_reserved_memory;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_atsr {
    pub Header: ACPI_DMAR_HEADER,
    pub Flags: u8,
    pub Reserved: u8,
    pub Segment: u16,
}
pub type ACPI_DMAR_ATSR = acpi_dmar_atsr;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_rhsa {
    pub Header: ACPI_DMAR_HEADER,
    pub Reserved: u32,
    pub BaseAddress: u64,
    pub ProximityDomain: u32,
}
pub type ACPI_DMAR_RHSA = acpi_dmar_rhsa;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_dmar_andd {
    pub Header: ACPI_DMAR_HEADER,
    pub Reserved: [u8; 3usize],
    pub DeviceNumber: u8,
    pub DeviceName: [i8; 1usize],
}
pub type ACPI_DMAR_ANDD = acpi_dmar_andd;