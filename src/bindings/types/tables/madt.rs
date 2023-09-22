use crate::bindings::types::FfiAcpiTableHeader;

use super::FfiAcpiSubtableHeader;

///  MADT - Multiple APIC Description Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableMadt {
    pub header: FfiAcpiTableHeader,
    pub address: u32,
    pub flags: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiMadtType {
    ACPI_MADT_TYPE_LOCAL_APIC = 0,
    ACPI_MADT_TYPE_IO_APIC = 1,
    ACPI_MADT_TYPE_INTERRUPT_OVERRIDE = 2,
    ACPI_MADT_TYPE_NMI_SOURCE = 3,
    ACPI_MADT_TYPE_LOCAL_APIC_NMI = 4,
    ACPI_MADT_TYPE_LOCAL_APIC_OVERRIDE = 5,
    ACPI_MADT_TYPE_IO_SAPIC = 6,
    ACPI_MADT_TYPE_LOCAL_SAPIC = 7,
    ACPI_MADT_TYPE_INTERRUPT_SOURCE = 8,
    ACPI_MADT_TYPE_LOCAL_X2APIC = 9,
    ACPI_MADT_TYPE_LOCAL_X2APIC_NMI = 10,
    ACPI_MADT_TYPE_GENERIC_INTERRUPT = 11,
    ACPI_MADT_TYPE_GENERIC_DISTRIBUTOR = 12,
    ACPI_MADT_TYPE_GENERIC_MSI_FRAME = 13,
    ACPI_MADT_TYPE_GENERIC_REDISTRIBUTOR = 14,
    ACPI_MADT_TYPE_GENERIC_TRANSLATOR = 15,
    ACPI_MADT_TYPE_MULTIPROC_WAKEUP = 16,
    ACPI_MADT_TYPE_RESERVED = 17,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalApic {
    pub header: FfiAcpiSubtableHeader,
    pub processor_id: u8,
    pub id: u8,
    pub lapic_flags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtIoApic {
    pub header: FfiAcpiSubtableHeader,
    pub id: u8,
    pub reserved: u8,
    pub address: u32,
    pub global_irq_base: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtInterruptOverride {
    pub header: FfiAcpiSubtableHeader,
    pub bus: u8,
    pub source_irq: u8,
    pub global_irq: u32,
    pub inti_flags: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtNmiSource {
    pub header: FfiAcpiSubtableHeader,
    pub inti_flags: u16,
    pub global_irq: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalApicNmi {
    pub header: FfiAcpiSubtableHeader,
    pub processor_id: u8,
    pub inti_flags: u16,
    pub lint: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalApicOverride {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub address: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtIoSapic {
    pub header: FfiAcpiSubtableHeader,
    pub id: u8,
    pub reserved: u8,
    pub global_irq_base: u32,
    pub address: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalSapic {
    pub header: FfiAcpiSubtableHeader,
    pub processor_id: u8,
    pub id: u8,
    pub eid: u8,
    pub reserved: [u8; 3usize],
    pub lapic_flags: u32,
    pub uid: u32,
    pub uid_string: [i8; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtInterruptSource {
    pub header: FfiAcpiSubtableHeader,
    pub inti_flags: u16,
    pub source_type: u8,
    pub id: u8,
    pub eid: u8,
    pub io_sapic_vector: u8,
    pub global_irq: u32,
    pub flags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalX2apic {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub local_apic_id: u32,
    pub lapic_flags: u32,
    pub uid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalX2apicNmi {
    pub header: FfiAcpiSubtableHeader,
    pub inti_flags: u16,
    pub uid: u32,
    pub lint: u8,
    pub reserved: [u8; 3usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericInterrupt {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub cpu_interface_number: u32,
    pub uid: u32,
    pub flags: u32,
    pub parking_version: u32,
    pub performance_interrupt: u32,
    pub parked_address: u64,
    pub base_address: u64,
    pub gicv_base_address: u64,
    pub gich_base_address: u64,
    pub vgic_interrupt: u32,
    pub gicr_base_address: u64,
    pub arm_mpidr: u64,
    pub efficiency_class: u8,
    pub reserved2: [u8; 1usize],
    pub spe_interrupt: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericDistributor {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub gic_id: u32,
    pub base_address: u64,
    pub global_irq_base: u32,
    pub version: u8,
    pub reserved2: [u8; 3usize],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiMadtGicVersion {
    ACPI_MADT_GIC_VERSION_NONE = 0,
    ACPI_MADT_GIC_VERSION_V1 = 1,
    ACPI_MADT_GIC_VERSION_V2 = 2,
    ACPI_MADT_GIC_VERSION_V3 = 3,
    ACPI_MADT_GIC_VERSION_V4 = 4,
    ACPI_MADT_GIC_VERSION_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericMsiFrame {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub msi_frame_id: u32,
    pub base_address: u64,
    pub flags: u32,
    pub spi_count: u16,
    pub spi_base: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericRedistributor {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub base_address: u64,
    pub length: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericTranslator {
    pub header: FfiAcpiSubtableHeader,
    pub reserved: u16,
    pub translation_id: u32,
    pub base_address: u64,
    pub reserved2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtMultiprocWakeup {
    pub header: FfiAcpiSubtableHeader,
    pub mailbox_version: u16,
    pub reserved: u32,
    pub base_address: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct FfiAcpiMadtMultiprocWakeupMailbox {
    pub command: u16,
    pub reserved: u16,
    pub apic_id: u32,
    pub wakeup_vector: u64,
    pub reserved_os: [u8; 2032usize],
    pub reserved_firmware: [u8; 2048usize],
}
