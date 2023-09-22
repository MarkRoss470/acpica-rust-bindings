use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::{__IncompleteArrayField, FfiAcpiWheaHeader}};

///  EINJ - Error Injection Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableEinj {
    pub header: FfiAcpiTableHeader,
    pub header_length: u32,
    pub flags: u8,
    pub reserved: [u8; 3usize],
    pub entries: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiEinjEntry {
    whea_header: FfiAcpiWheaHeader,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiEinjActions {
    ACPI_EINJ_BEGIN_OPERATION = 0,
    ACPI_EINJ_GET_TRIGGER_TABLE = 1,
    ACPI_EINJ_SET_ERROR_TYPE = 2,
    ACPI_EINJ_GET_ERROR_TYPE = 3,
    ACPI_EINJ_END_OPERATION = 4,
    ACPI_EINJ_EXECUTE_OPERATION = 5,
    ACPI_EINJ_CHECK_BUSY_STATUS = 6,
    ACPI_EINJ_GET_COMMAND_STATUS = 7,
    ACPI_EINJ_SET_ERROR_TYPE_WITH_ADDRESS = 8,
    ACPI_EINJ_GET_EXECUTE_TIMINGS = 9,
    ACPI_EINJ_ACTION_RESERVED = 10,
    ACPI_EINJ_TRIGGER_ERROR = 255,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiEinjInstructions {
    ACPI_EINJ_READ_REGISTER = 0,
    ACPI_EINJ_READ_REGISTER_VALUE = 1,
    ACPI_EINJ_WRITE_REGISTER = 2,
    ACPI_EINJ_WRITE_REGISTER_VALUE = 3,
    ACPI_EINJ_NOOP = 4,
    ACPI_EINJ_FLUSH_CACHELINE = 5,
    ACPI_EINJ_INSTRUCTION_RESERVED = 6,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiEinjErrorTypeWithAddr {
    pub error_type: u32,
    pub vendor_struct_offset: u32,
    pub flags: u32,
    pub apic_id: u32,
    pub address: u64,
    pub range: u64,
    pub pcie_id: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiEinjVendor {
    pub length: u32,
    pub pcie_id: u32,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u8,
    pub reserved: [u8; 3usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiEinjTrigger {
    pub header_size: u32,
    pub revision: u32,
    pub table_size: u32,
    pub entry_count: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiEinjCommandStatus {
    ACPI_EINJ_SUCCESS = 0,
    ACPI_EINJ_FAILURE = 1,
    ACPI_EINJ_INVALID_ACCESS = 2,
    ACPI_EINJ_STATUS_RESERVED = 3,
}