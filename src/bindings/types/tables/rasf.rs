use crate::bindings::types::FfiAcpiTableHeader;


///  RASF - RAS Feature Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableRasf {
    pub header: FfiAcpiTableHeader,
    pub channel_id: [u8; 12usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiRasfSharedMemory {
    pub signature: u32,
    pub command: u16,
    pub status: u16,
    pub version: u16,
    pub capabilities: [u8; 16usize],
    pub set_capabilities: [u8; 16usize],
    pub num_parameter_blocks: u16,
    pub set_capabilities_status: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiRasfParameterBlock {
    pub block_type: u16,
    pub version: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiRasfPatrolScrubParameter {
    pub header: FfiAcpiRasfParameterBlock,
    pub patrol_scrub_command: u16,
    pub requested_address_range: [u64; 2usize],
    pub actual_address_range: [u64; 2usize],
    pub flags: u16,
    pub requested_speed: u8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiRasfCommands {
    ACPI_RASF_EXECUTE_RASF_COMMAND = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiRasfCapabiliities {
    ACPI_HW_PATROL_SCRUB_SUPPORTED = 0,
    ACPI_SW_PATROL_SCRUB_EXPOSED = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiRasfPatrolScrubCommands {
    ACPI_RASF_GET_PATROL_PARAMETERS = 1,
    ACPI_RASF_START_PATROL_SCRUBBER = 2,
    ACPI_RASF_STOP_PATROL_SCRUBBER = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiRasfStatus {
    ACPI_RASF_SUCCESS = 0,
    ACPI_RASF_NOT_VALID = 1,
    ACPI_RASF_NOT_SUPPORTED = 2,
    ACPI_RASF_BUSY = 3,
    ACPI_RASF_FAILED = 4,
    ACPI_RASF_ABORTED = 5,
    ACPI_RASF_INVALID_DATA = 6,
}