use crate::bindings::types::FfiAcpiTableHeader;

use super::FfiAcpiSubtableHeader;

///  MADT - Multiple APIC Description Table
///         Version 3
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableMadt {
    pub Header: FfiAcpiTableHeader,
    pub Address: u32,
    pub Flags: u32,
}
///  MADT - Multiple APIC Description Table
///         Version 3
/// 

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
    pub Header: FfiAcpiSubtableHeader,
    pub ProcessorId: u8,
    pub Id: u8,
    pub LapicFlags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtIoApic {
    pub Header: FfiAcpiSubtableHeader,
    pub Id: u8,
    pub Reserved: u8,
    pub Address: u32,
    pub GlobalIrqBase: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtInterruptOverride {
    pub Header: FfiAcpiSubtableHeader,
    pub Bus: u8,
    pub SourceIrq: u8,
    pub GlobalIrq: u32,
    pub IntiFlags: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtNmiSource {
    pub Header: FfiAcpiSubtableHeader,
    pub IntiFlags: u16,
    pub GlobalIrq: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalApicNmi {
    pub Header: FfiAcpiSubtableHeader,
    pub ProcessorId: u8,
    pub IntiFlags: u16,
    pub Lint: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalApicOverride {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub Address: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtIoSapic {
    pub Header: FfiAcpiSubtableHeader,
    pub Id: u8,
    pub Reserved: u8,
    pub GlobalIrqBase: u32,
    pub Address: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalSapic {
    pub Header: FfiAcpiSubtableHeader,
    pub ProcessorId: u8,
    pub Id: u8,
    pub Eid: u8,
    pub Reserved: [u8; 3usize],
    pub LapicFlags: u32,
    pub Uid: u32,
    pub UidString: [i8; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtInterruptSource {
    pub Header: FfiAcpiSubtableHeader,
    pub IntiFlags: u16,
    pub Type: u8,
    pub Id: u8,
    pub Eid: u8,
    pub IoSapicVector: u8,
    pub GlobalIrq: u32,
    pub Flags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalX2apic {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub LocalApicId: u32,
    pub LapicFlags: u32,
    pub Uid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtLocalX2apicNmi {
    pub Header: FfiAcpiSubtableHeader,
    pub IntiFlags: u16,
    pub Uid: u32,
    pub Lint: u8,
    pub Reserved: [u8; 3usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericInterrupt {
    pub Header: FfiAcpiSubtableHeader,
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

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericDistributor {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub GicId: u32,
    pub BaseAddress: u64,
    pub GlobalIrqBase: u32,
    pub Version: u8,
    pub Reserved2: [u8; 3usize],
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
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub MsiFrameId: u32,
    pub BaseAddress: u64,
    pub Flags: u32,
    pub SpiCount: u16,
    pub SpiBase: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericRedistributor {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub BaseAddress: u64,
    pub Length: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtGenericTranslator {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: u16,
    pub TranslationId: u32,
    pub BaseAddress: u64,
    pub Reserved2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMadtMultiprocWakeup {
    pub Header: FfiAcpiSubtableHeader,
    pub MailboxVersion: u16,
    pub Reserved: u32,
    pub BaseAddress: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct FfiAcpiMadtMultiprocWakeupMailbox {
    pub Command: u16,
    pub Reserved: u16,
    pub ApicId: u32,
    pub WakeupVector: u64,
    pub ReservedOs: [u8; 2032usize],
    pub ReservedFirmware: [u8; 2048usize],
}
