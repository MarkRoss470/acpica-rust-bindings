use crate::bindings::types::FfiAcpiTableHeader;


///  RASF - RAS Feature Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableRasf {
    pub Header: FfiAcpiTableHeader,
    pub ChannelId: [u8; 12usize],
}
///  RASF - RAS Feature Table (ACPI 5.0)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiRasfSharedMemory {
    pub Signature: u32,
    pub Command: u16,
    pub Status: u16,
    pub Version: u16,
    pub Capabilities: [u8; 16usize],
    pub SetCapabilities: [u8; 16usize],
    pub NumParameterBlocks: u16,
    pub SetCapabilitiesStatus: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiRasfParameterBlock {
    pub Type: u16,
    pub Version: u16,
    pub Length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiRasfPatrolScrubParameter {
    pub Header: FfiAcpiRasfParameterBlock,
    pub PatrolScrubCommand: u16,
    pub RequestedAddressRange: [u64; 2usize],
    pub ActualAddressRange: [u64; 2usize],
    pub Flags: u16,
    pub RequestedSpeed: u8,
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