use crate::types::AcpiTableHeader;

use super::ACPI_SUBTABLE_HEADER;

///  MADT - Multiple APIC Description Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_madt {
    pub Header: AcpiTableHeader,
    pub Address: u32,
    pub Flags: u32,
}
///  MADT - Multiple APIC Description Table
///         Version 3
/// 
pub type ACPI_TABLE_MADT = acpi_table_madt;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiMadtType {
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
pub struct acpi_madt_local_apic {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProcessorId: u8,
    pub Id: u8,
    pub LapicFlags: u32,
}
pub type ACPI_MADT_LOCAL_APIC = acpi_madt_local_apic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_io_apic {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Id: u8,
    pub Reserved: u8,
    pub Address: u32,
    pub GlobalIrqBase: u32,
}
pub type ACPI_MADT_IO_APIC = acpi_madt_io_apic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_interrupt_override {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Bus: u8,
    pub SourceIrq: u8,
    pub GlobalIrq: u32,
    pub IntiFlags: u16,
}
pub type ACPI_MADT_INTERRUPT_OVERRIDE = acpi_madt_interrupt_override;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_nmi_source {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub IntiFlags: u16,
    pub GlobalIrq: u32,
}
pub type ACPI_MADT_NMI_SOURCE = acpi_madt_nmi_source;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_local_apic_nmi {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProcessorId: u8,
    pub IntiFlags: u16,
    pub Lint: u8,
}
pub type ACPI_MADT_LOCAL_APIC_NMI = acpi_madt_local_apic_nmi;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_local_apic_override {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub Address: u64,
}
pub type ACPI_MADT_LOCAL_APIC_OVERRIDE = acpi_madt_local_apic_override;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_io_sapic {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Id: u8,
    pub Reserved: u8,
    pub GlobalIrqBase: u32,
    pub Address: u64,
}
pub type ACPI_MADT_IO_SAPIC = acpi_madt_io_sapic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_local_sapic {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub ProcessorId: u8,
    pub Id: u8,
    pub Eid: u8,
    pub Reserved: [u8; 3usize],
    pub LapicFlags: u32,
    pub Uid: u32,
    pub UidString: [i8; 1usize],
}
pub type ACPI_MADT_LOCAL_SAPIC = acpi_madt_local_sapic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_interrupt_source {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub IntiFlags: u16,
    pub Type: u8,
    pub Id: u8,
    pub Eid: u8,
    pub IoSapicVector: u8,
    pub GlobalIrq: u32,
    pub Flags: u32,
}
pub type ACPI_MADT_INTERRUPT_SOURCE = acpi_madt_interrupt_source;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_local_x2apic {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub LocalApicId: u32,
    pub LapicFlags: u32,
    pub Uid: u32,
}
pub type ACPI_MADT_LOCAL_X2APIC = acpi_madt_local_x2apic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_local_x2apic_nmi {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub IntiFlags: u16,
    pub Uid: u32,
    pub Lint: u8,
    pub Reserved: [u8; 3usize],
}
pub type ACPI_MADT_LOCAL_X2APIC_NMI = acpi_madt_local_x2apic_nmi;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_generic_interrupt {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub CpuInterfaceNumber: u32,
    pub Uid: u32,
    pub Flags: u32,
    pub ParkingVersion: u32,
    pub PerformanceInterrupt: u32,
    pub ParkedAddress: u64,
    pub BaseAddress: u64,
    pub GicvBaseAddress: u64,
    pub GichBaseAddress: u64,
    pub VgicInterrupt: u32,
    pub GicrBaseAddress: u64,
    pub ArmMpidr: u64,
    pub EfficiencyClass: u8,
    pub Reserved2: [u8; 1usize],
    pub SpeInterrupt: u16,
}
pub type ACPI_MADT_GENERIC_INTERRUPT = acpi_madt_generic_interrupt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_generic_distributor {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub GicId: u32,
    pub BaseAddress: u64,
    pub GlobalIrqBase: u32,
    pub Version: u8,
    pub Reserved2: [u8; 3usize],
}
pub type ACPI_MADT_GENERIC_DISTRIBUTOR = acpi_madt_generic_distributor;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiMadtGicVersion {
    ACPI_MADT_GIC_VERSION_NONE = 0,
    ACPI_MADT_GIC_VERSION_V1 = 1,
    ACPI_MADT_GIC_VERSION_V2 = 2,
    ACPI_MADT_GIC_VERSION_V3 = 3,
    ACPI_MADT_GIC_VERSION_V4 = 4,
    ACPI_MADT_GIC_VERSION_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_generic_msi_frame {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub MsiFrameId: u32,
    pub BaseAddress: u64,
    pub Flags: u32,
    pub SpiCount: u16,
    pub SpiBase: u16,
}
pub type ACPI_MADT_GENERIC_MSI_FRAME = acpi_madt_generic_msi_frame;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_generic_redistributor {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub BaseAddress: u64,
    pub Length: u32,
}
pub type ACPI_MADT_GENERIC_REDISTRIBUTOR = acpi_madt_generic_redistributor;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_generic_translator {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Reserved: u16,
    pub TranslationId: u32,
    pub BaseAddress: u64,
    pub Reserved2: u32,
}
pub type ACPI_MADT_GENERIC_TRANSLATOR = acpi_madt_generic_translator;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_madt_multiproc_wakeup {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub MailboxVersion: u16,
    pub Reserved: u32,
    pub BaseAddress: u64,
}
pub type ACPI_MADT_MULTIPROC_WAKEUP = acpi_madt_multiproc_wakeup;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct acpi_madt_multiproc_wakeup_mailbox {
    pub Command: u16,
    pub Reserved: u16,
    pub ApicId: u32,
    pub WakeupVector: u64,
    pub ReservedOs: [u8; 2032usize],
    pub ReservedFirmware: [u8; 2048usize],
}
pub type ACPI_MADT_MULTIPROC_WAKEUP_MAILBOX = acpi_madt_multiproc_wakeup_mailbox;