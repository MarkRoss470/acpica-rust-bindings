use crate::bindings::types::FfiAcpiTableHeader;

///  VIOT - Virtual I/O Translation Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_viot {
    pub Header: FfiAcpiTableHeader,
    pub NodeCount: u16,
    pub NodeOffset: u16,
    pub Reserved: [u8; 8usize],
}
///  VIOT - Virtual I/O Translation Table
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_header {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiViotNodeType {
    ACPI_VIOT_NODE_PCI_RANGE = 1,
    ACPI_VIOT_NODE_MMIO = 2,
    ACPI_VIOT_NODE_VIRTIO_IOMMU_PCI = 3,
    ACPI_VIOT_NODE_VIRTIO_IOMMU_MMIO = 4,
    ACPI_VIOT_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_pci_range {
    pub Header: acpi_viot_header,
    pub EndpointStart: u32,
    pub SegmentStart: u16,
    pub SegmentEnd: u16,
    pub BdfStart: u16,
    pub BdfEnd: u16,
    pub OutputNode: u16,
    pub Reserved: [u8; 6usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_mmio {
    pub Header: acpi_viot_header,
    pub Endpoint: u32,
    pub BaseAddress: u64,
    pub OutputNode: u16,
    pub Reserved: [u8; 6usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_virtio_iommu_pci {
    pub Header: acpi_viot_header,
    pub Segment: u16,
    pub Bdf: u16,
    pub Reserved: [u8; 8usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_virtio_iommu_mmio {
    pub Header: acpi_viot_header,
    pub Reserved: [u8; 4usize],
    pub BaseAddress: u64,
}
