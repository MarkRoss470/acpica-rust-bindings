use crate::types::AcpiTableHeader;

///  VIOT - Virtual I/O Translation Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_viot {
    pub Header: AcpiTableHeader,
    pub NodeCount: u16,
    pub NodeOffset: u16,
    pub Reserved: [u8; 8usize],
}
///  VIOT - Virtual I/O Translation Table
///         Version 1
/// 
pub type ACPI_TABLE_VIOT = acpi_table_viot;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_header {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}
pub type ACPI_VIOT_HEADER = acpi_viot_header;
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
    pub Header: ACPI_VIOT_HEADER,
    pub EndpointStart: u32,
    pub SegmentStart: u16,
    pub SegmentEnd: u16,
    pub BdfStart: u16,
    pub BdfEnd: u16,
    pub OutputNode: u16,
    pub Reserved: [u8; 6usize],
}
pub type ACPI_VIOT_PCI_RANGE = acpi_viot_pci_range;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_mmio {
    pub Header: ACPI_VIOT_HEADER,
    pub Endpoint: u32,
    pub BaseAddress: u64,
    pub OutputNode: u16,
    pub Reserved: [u8; 6usize],
}
pub type ACPI_VIOT_MMIO = acpi_viot_mmio;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_virtio_iommu_pci {
    pub Header: ACPI_VIOT_HEADER,
    pub Segment: u16,
    pub Bdf: u16,
    pub Reserved: [u8; 8usize],
}
pub type ACPI_VIOT_VIRTIO_IOMMU_PCI = acpi_viot_virtio_iommu_pci;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_viot_virtio_iommu_mmio {
    pub Header: ACPI_VIOT_HEADER,
    pub Reserved: [u8; 4usize],
    pub BaseAddress: u64,
}
pub type ACPI_VIOT_VIRTIO_IOMMU_MMIO = acpi_viot_virtio_iommu_mmio;