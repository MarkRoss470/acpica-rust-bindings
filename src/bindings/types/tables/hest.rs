use crate::{bindings::types::FfiAcpiTableHeader, interface::AcpiGenericAddress};

///  HEST - Hardware Error Source Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableHest {
    pub header: FfiAcpiTableHeader,
    pub error_source_count: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestHeader {
    pub header_type: u16,
    pub source_id: u16,
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
    pub bank_number: u8,
    pub clear_status_on_init: u8,
    pub status_format: u8,
    pub reserved: u8,
    pub control_register: u32,
    pub control_data: u64,
    pub status_register: u32,
    pub address_register: u32,
    pub misc_register: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAerCommon {
    pub reserved1: u16,
    pub flags: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub bus: u32,
    pub device: u16,
    pub function: u16,
    pub device_control: u16,
    pub reserved2: u16,
    pub uncorrectable_mask: u32,
    pub uncorrectable_severity: u32,
    pub correctable_mask: u32,
    pub advanced_capabilities: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestNotify {
    pub notify_type: u8,
    pub length: u8,
    pub config_write_enable: u16,
    pub poll_interval: u32,
    pub vector: u32,
    pub polling_threshold_value: u32,
    pub polling_threshold_window: u32,
    pub error_threshold_value: u32,
    pub error_threshold_window: u32,
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
    pub header: FfiAcpiHestHeader,
    pub reserved1: u16,
    pub flags: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub global_capability_data: u64,
    pub global_control_data: u64,
    pub num_hardware_banks: u8,
    pub reserved3: [u8; 7usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaCorrected {
    pub header: FfiAcpiHestHeader,
    pub reserved1: u16,
    pub flags: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub notify: FfiAcpiHestNotify,
    pub num_hardware_banks: u8,
    pub reserved2: [u8; 3usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaNmi {
    pub header: FfiAcpiHestHeader,
    pub reserved: u32,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub max_raw_data_length: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAerRoot {
    pub header: FfiAcpiHestHeader,
    pub aer: FfiAcpiHestAerCommon,
    pub root_error_command: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAer {
    pub header: FfiAcpiHestHeader,
    pub aer: FfiAcpiHestAerCommon,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestAerBridge {
    pub header: FfiAcpiHestHeader,
    pub aer: FfiAcpiHestAerCommon,
    pub uncorrectable_mask2: u32,
    pub uncorrectable_severity2: u32,
    pub advanced_capabilities2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGeneric {
    pub header: FfiAcpiHestHeader,
    pub related_source_id: u16,
    pub reserved: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub max_raw_data_length: u32,
    pub error_status_address: AcpiGenericAddress,
    pub notify: FfiAcpiHestNotify,
    pub error_block_length: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericV2 {
    pub header: FfiAcpiHestHeader,
    pub related_source_id: u16,
    pub reserved: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub max_raw_data_length: u32,
    pub error_status_address: AcpiGenericAddress,
    pub notify: FfiAcpiHestNotify,
    pub error_block_length: u32,
    pub read_ack_register: AcpiGenericAddress,
    pub read_ack_preserve: u64,
    pub read_ack_write: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericStatus {
    pub block_status: u32,
    pub raw_data_offset: u32,
    pub raw_data_length: u32,
    pub data_length: u32,
    pub error_severity: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericData {
    pub section_type: [u8; 16usize],
    pub error_severity: u32,
    pub revision: u16,
    pub validation_bits: u8,
    pub flags: u8,
    pub error_data_length: u32,
    pub fru_id: [u8; 16usize],
    pub fru_text: [u8; 20usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestGenericDataV300 {
    pub section_type: [u8; 16usize],
    pub error_severity: u32,
    pub revision: u16,
    pub validation_bits: u8,
    pub flags: u8,
    pub error_data_length: u32,
    pub fru_id: [u8; 16usize],
    pub fru_text: [u8; 20usize],
    pub time_stamp: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiHestIaDeferredCheck {
    pub header: FfiAcpiHestHeader,
    pub reserved1: u16,
    pub flags: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub notify: FfiAcpiHestNotify,
    pub num_hardware_banks: u8,
    pub reserved2: [u8; 3usize],
}