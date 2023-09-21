use crate::{types::AcpiTableHeader, bindings::types::ACPI_WHEA_HEADER};



#[doc = " ERST - Error Record Serialization Table (ACPI 4.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_erst {
    pub Header: AcpiTableHeader,
    pub HeaderLength: u32,
    pub Reserved: u32,
    pub Entries: u32,
}
#[doc = " ERST - Error Record Serialization Table (ACPI 4.0)"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_ERST = acpi_table_erst;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_erst_entry {
    WheaHeader: ACPI_WHEA_HEADER,
}
pub type ACPI_ERST_ENTRY = acpi_erst_entry;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiErstActions {
    ACPI_ERST_BEGIN_WRITE = 0,
    ACPI_ERST_BEGIN_READ = 1,
    ACPI_ERST_BEGIN_CLEAR = 2,
    ACPI_ERST_END = 3,
    ACPI_ERST_SET_RECORD_OFFSET = 4,
    ACPI_ERST_EXECUTE_OPERATION = 5,
    ACPI_ERST_CHECK_BUSY_STATUS = 6,
    ACPI_ERST_GET_COMMAND_STATUS = 7,
    ACPI_ERST_GET_RECORD_ID = 8,
    ACPI_ERST_SET_RECORD_ID = 9,
    ACPI_ERST_GET_RECORD_COUNT = 10,
    ACPI_ERST_BEGIN_DUMMY_WRIITE = 11,
    ACPI_ERST_NOT_USED = 12,
    ACPI_ERST_GET_ERROR_RANGE = 13,
    ACPI_ERST_GET_ERROR_LENGTH = 14,
    ACPI_ERST_GET_ERROR_ATTRIBUTES = 15,
    ACPI_ERST_EXECUTE_TIMINGS = 16,
    ACPI_ERST_ACTION_RESERVED = 17,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiErstInstructions {
    ACPI_ERST_READ_REGISTER = 0,
    ACPI_ERST_READ_REGISTER_VALUE = 1,
    ACPI_ERST_WRITE_REGISTER = 2,
    ACPI_ERST_WRITE_REGISTER_VALUE = 3,
    ACPI_ERST_NOOP = 4,
    ACPI_ERST_LOAD_VAR1 = 5,
    ACPI_ERST_LOAD_VAR2 = 6,
    ACPI_ERST_STORE_VAR1 = 7,
    ACPI_ERST_ADD = 8,
    ACPI_ERST_SUBTRACT = 9,
    ACPI_ERST_ADD_VALUE = 10,
    ACPI_ERST_SUBTRACT_VALUE = 11,
    ACPI_ERST_STALL = 12,
    ACPI_ERST_STALL_WHILE_TRUE = 13,
    ACPI_ERST_SKIP_NEXT_IF_TRUE = 14,
    ACPI_ERST_GOTO = 15,
    ACPI_ERST_SET_SRC_ADDRESS_BASE = 16,
    ACPI_ERST_SET_DST_ADDRESS_BASE = 17,
    ACPI_ERST_MOVE_DATA = 18,
    ACPI_ERST_INSTRUCTION_RESERVED = 19,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiErstCommandStatus {
    ACPI_ERST_SUCCESS = 0,
    ACPI_ERST_NO_SPACE = 1,
    ACPI_ERST_NOT_AVAILABLE = 2,
    ACPI_ERST_FAILURE = 3,
    ACPI_ERST_RECORD_EMPTY = 4,
    ACPI_ERST_NOT_FOUND = 5,
    ACPI_ERST_STATUS_RESERVED = 6,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct acpi_erst_info {
    pub Signature: u16,
    pub Data: [u8; 48usize],
}
pub type ACPI_ERST_INFO = acpi_erst_info;