#[doc = " HEST - Hardware Error Source Table (ACPI 4.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_hest {
    pub Header: AcpiTableHeader,
    pub ErrorSourceCount: u32,
}
#[doc = " HEST - Hardware Error Source Table (ACPI 4.0)"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_HEST = acpi_table_hest;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_header {
    pub Type: u16,
    pub SourceId: u16,
}
pub type ACPI_HEST_HEADER = acpi_hest_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiHestTypes {
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
pub struct acpi_hest_ia_error_bank {
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
pub type ACPI_HEST_IA_ERROR_BANK = acpi_hest_ia_error_bank;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_aer_common {
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
pub type ACPI_HEST_AER_COMMON = acpi_hest_aer_common;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_notify {
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
pub type ACPI_HEST_NOTIFY = acpi_hest_notify;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiHestNotifyTypes {
    ACPI_HEST_NOTIFY_POLLED = 0,
    ACPI_HEST_NOTIFY_EXTERNAL = 1,
    ACPI_HEST_NOTIFY_LOCAL = 2,
    ACPI_HEST_NOTIFY_SCI = 3,
    ACPI_HEST_NOTIFY_NMI = 4,
    ACPI_HEST_NOTIFY_CMCI = 5,
    ACPI_HEST_NOTIFY_MCE = 6,
    ACPI_HEST_NOTIFY_GPIO = 7,
    ACPI_HEST_NOTIFY_SEA = 8,
    ACPI_HEST_NOTIFY_SEI = 9,
    ACPI_HEST_NOTIFY_GSIV = 10,
    ACPI_HEST_NOTIFY_SOFTWARE_DELEGATED = 11,
    ACPI_HEST_NOTIFY_RESERVED = 12,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_ia_machine_check {
    pub Header: ACPI_HEST_HEADER,
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
pub type ACPI_HEST_IA_MACHINE_CHECK = acpi_hest_ia_machine_check;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_ia_corrected {
    pub Header: ACPI_HEST_HEADER,
    pub Reserved1: u16,
    pub Flags: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub Notify: ACPI_HEST_NOTIFY,
    pub NumHardwareBanks: u8,
    pub Reserved2: [u8; 3usize],
}
pub type ACPI_HEST_IA_CORRECTED = acpi_hest_ia_corrected;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_ia_nmi {
    pub Header: ACPI_HEST_HEADER,
    pub Reserved: u32,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub MaxRawDataLength: u32,
}
pub type ACPI_HEST_IA_NMI = acpi_hest_ia_nmi;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_aer_root {
    pub Header: ACPI_HEST_HEADER,
    pub Aer: ACPI_HEST_AER_COMMON,
    pub RootErrorCommand: u32,
}
pub type ACPI_HEST_AER_ROOT = acpi_hest_aer_root;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_aer {
    pub Header: ACPI_HEST_HEADER,
    pub Aer: ACPI_HEST_AER_COMMON,
}
pub type ACPI_HEST_AER = acpi_hest_aer;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_aer_bridge {
    pub Header: ACPI_HEST_HEADER,
    pub Aer: ACPI_HEST_AER_COMMON,
    pub UncorrectableMask2: u32,
    pub UncorrectableSeverity2: u32,
    pub AdvancedCapabilities2: u32,
}
pub type ACPI_HEST_AER_BRIDGE = acpi_hest_aer_bridge;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_generic {
    pub Header: ACPI_HEST_HEADER,
    pub RelatedSourceId: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub MaxRawDataLength: u32,
    pub ErrorStatusAddress: ACPI_GENERIC_ADDRESS,
    pub Notify: ACPI_HEST_NOTIFY,
    pub ErrorBlockLength: u32,
}
pub type ACPI_HEST_GENERIC = acpi_hest_generic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_generic_v2 {
    pub Header: ACPI_HEST_HEADER,
    pub RelatedSourceId: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub MaxRawDataLength: u32,
    pub ErrorStatusAddress: ACPI_GENERIC_ADDRESS,
    pub Notify: ACPI_HEST_NOTIFY,
    pub ErrorBlockLength: u32,
    pub ReadAckRegister: ACPI_GENERIC_ADDRESS,
    pub ReadAckPreserve: u64,
    pub ReadAckWrite: u64,
}
pub type ACPI_HEST_GENERIC_V2 = acpi_hest_generic_v2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_generic_status {
    pub BlockStatus: u32,
    pub RawDataOffset: u32,
    pub RawDataLength: u32,
    pub DataLength: u32,
    pub ErrorSeverity: u32,
}
pub type ACPI_HEST_GENERIC_STATUS = acpi_hest_generic_status;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_generic_data {
    pub SectionType: [u8; 16usize],
    pub ErrorSeverity: u32,
    pub Revision: u16,
    pub ValidationBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FruId: [u8; 16usize],
    pub FruText: [u8; 20usize],
}
pub type ACPI_HEST_GENERIC_DATA = acpi_hest_generic_data;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_generic_data_v300 {
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
pub type ACPI_HEST_GENERIC_DATA_V300 = acpi_hest_generic_data_v300;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_hest_ia_deferred_check {
    pub Header: ACPI_HEST_HEADER,
    pub Reserved1: u16,
    pub Flags: u8,
    pub Enabled: u8,
    pub RecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub Notify: ACPI_HEST_NOTIFY,
    pub NumHardwareBanks: u8,
    pub Reserved2: [u8; 3usize],
}
pub type ACPI_HEST_IA_DEFERRED_CHECK = acpi_hest_ia_deferred_check;