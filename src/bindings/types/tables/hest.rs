use crate::{bindings::types::FfiAcpiTableHeader, interface::AcpiGenericAddress};

///  HEST - Hardware Error Source Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableHest {
    pub Header: FfiAcpiTableHeader,
    pub ErrorSourceCount: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestHeader {
    pub Type: u16,
    pub SourceId: u16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiHestTypes {
    ACPI_HEST_TYPE_IA32_CHECK = 0,
    ACPI_HEST_TYPE_IA32_CORRECTED_CHECK = 1,
    ACPI_HEST_TYPE_IA32_NMI = 2,
    ACPI_HEST_TYPE_NOT_USED3 = 3,
    ACPI_HEST_TYPE_NOT_USED4 = 4,
    ACPI_HEST_TYPE_NOT_USED5 = 5,
    ACPI_HEST_TYPE_AER_ROOT_PORT = 6,
    ACPI_HEST_TYPE_AER_ENDPOINT = 7,
    ACPI_HEST_TYPE_AER_BRIDGE = 8,
    ACPI_HEST_TYPE_GENERIC_ERROR = 9,
    ACPI_HEST_TYPE_GENERIC_ERROR_V2 = 10,
    ACPI_HEST_TYPE_IA32_DEFERRED_CHECK = 11,
    ACPI_HEST_TYPE_RESERVED = 12,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaErrorBank {
    pub BankNumber: u8,
    pub ClearStatusOnInit: u8,
    pub StatusFormat: u8,
    pub Reserved: u8,
    pub ControlRegister: u32,
    pub ControlData: u64,
    pub StatusRegister: u32,
    pub AddressRegister: u32,
    pub MiscRegister: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAerCommon {
    pub Reserved1: u16,
    pub Flags: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub Bus: u32,
    pub Device: u16,
    pub Function: u16,
    pub DeviceControl: u16,
    pub Reserved2: u16,
    pub UncorrectableMask: u32,
    pub UncorrectableSeverity: u32,
    pub CorrectableMask: u32,
    pub AdvancedCapabilities: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestNotify {
    pub Type: u8,
    pub Length: u8,
    pub ConfigWriteEnable: u16,
    pub PollInterval: u32,
    pub Vector: u32,
    pub PollingThresholdValue: u32,
    pub PollingThresholdWindow: u32,
    pub ErrorThresholdValue: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiHestNotifyTypes {
    acpi_hest_notify_POLLED = 0,
    acpi_hest_notify_EXTERNAL = 1,
    acpi_hest_notify_LOCAL = 2,
    acpi_hest_notify_SCI = 3,
    acpi_hest_notify_NMI = 4,
    acpi_hest_notify_CMCI = 5,
    acpi_hest_notify_MCE = 6,
    acpi_hest_notify_GPIO = 7,
    acpi_hest_notify_SEA = 8,
    acpi_hest_notify_SEI = 9,
    acpi_hest_notify_GSIV = 10,
    acpi_hest_notify_SOFTWARE_DELEGATED = 11,
    acpi_hest_notify_RESERVED = 12,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaMachineCheck {
    pub Header: FfiAcpiHestHeader,
    pub Reserved1: u16,
    pub Flags: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub GlobalCapabilityData: u64,
    pub GlobalControlData: u64,
    pub NumHardwareBanks: u8,
    pub Reserved3: [u8; 7usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaCorrected {
    pub Header: FfiAcpiHestHeader,
    pub Reserved1: u16,
    pub Flags: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub Notify: FfiAcpiHestNotify,
    pub NumHardwareBanks: u8,
    pub Reserved2: [u8; 3usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaNmi {
    pub Header: FfiAcpiHestHeader,
    pub Reserved: u32,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub MaxRawDataLength: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAerRoot {
    pub Header: FfiAcpiHestHeader,
    pub Aer: FfiAcpiHestAerCommon,
    pub RootErrorCommand: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAer {
    pub Header: FfiAcpiHestHeader,
    pub Aer: FfiAcpiHestAerCommon,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAerBridge {
    pub Header: FfiAcpiHestHeader,
    pub Aer: FfiAcpiHestAerCommon,
    pub UncorrectableMask2: u32,
    pub UncorrectableSeverity2: u32,
    pub AdvancedCapabilities2: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGeneric {
    pub Header: FfiAcpiHestHeader,
    pub RelatedSourceId: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub MaxRawDataLength: u32,
    pub ErrorStatusAddress: AcpiGenericAddress,
    pub Notify: FfiAcpiHestNotify,
    pub ErrorBlockLength: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericV2 {
    pub Header: FfiAcpiHestHeader,
    pub RelatedSourceId: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub MaxRawDataLength: u32,
    pub ErrorStatusAddress: AcpiGenericAddress,
    pub Notify: FfiAcpiHestNotify,
    pub ErrorBlockLength: u32,
    pub ReadAckRegister: AcpiGenericAddress,
    pub ReadAckPreserve: u64,
    pub ReadAckWrite: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericStatus {
    pub BlockStatus: u32,
    pub RawDataOffset: u32,
    pub RawDataLength: u32,
    pub DataLength: u32,
    pub ErrorSeverity: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericData {
    pub SectionType: [u8; 16usize],
    pub ErrorSeverity: u32,
    pub Revision: u16,
    pub ValidationBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FruId: [u8; 16usize],
    pub FruText: [u8; 20usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericDataV300 {
    pub SectionType: [u8; 16usize],
    pub ErrorSeverity: u32,
    pub Revision: u16,
    pub ValidationBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FruId: [u8; 16usize],
    pub FruText: [u8; 20usize],
    pub TimeStamp: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaDeferredCheck {
    pub Header: FfiAcpiHestHeader,
    pub Reserved1: u16,
    pub Flags: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub Notify: FfiAcpiHestNotify,
    pub NumHardwareBanks: u8,
    pub Reserved2: [u8; 3usize],
}