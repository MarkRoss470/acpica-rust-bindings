use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::{__IncompleteArrayField, ACPI_WHEA_HEADER}};

///  EINJ - Error Injection Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_einj {
    pub Header: FfiAcpiTableHeader,
    pub HeaderLength: u32,
    pub Flags: u8,
    pub Reserved: [u8; 3usize],
    pub Entries: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_einj_entry {
    WheaHeader: ACPI_WHEA_HEADER,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiEinjActions {
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
pub enum AcpiEinjInstructions {
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
pub struct acpi_einj_error_type_with_addr {
    pub ErrorType: u32,
    pub VendorStructOffset: u32,
    pub Flags: u32,
    pub ApicId: u32,
    pub Address: u64,
    pub Range: u64,
    pub PcieId: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_einj_vendor {
    pub Length: u32,
    pub PcieId: u32,
    pub VendorId: u16,
    pub DeviceId: u16,
    pub RevisionId: u8,
    pub Reserved: [u8; 3usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_einj_trigger {
    pub HeaderSize: u32,
    pub Revision: u32,
    pub TableSize: u32,
    pub EntryCount: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiEinjCommandStatus {
    ACPI_EINJ_SUCCESS = 0,
    ACPI_EINJ_FAILURE = 1,
    ACPI_EINJ_INVALID_ACCESS = 2,
    ACPI_EINJ_STATUS_RESERVED = 3,
}