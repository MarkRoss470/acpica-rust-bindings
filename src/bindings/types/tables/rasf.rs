
#[doc = " RASF - RAS Feature Table (ACPI 5.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_rasf {
    pub Header: AcpiTableHeader,
    pub ChannelId: [u8; 12usize],
}
#[doc = " RASF - RAS Feature Table (ACPI 5.0)"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_RASF = acpi_table_rasf;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_rasf_shared_memory {
    pub Signature: u32,
    pub Command: u16,
    pub Status: u16,
    pub Version: u16,
    pub Capabilities: [u8; 16usize],
    pub SetCapabilities: [u8; 16usize],
    pub NumParameterBlocks: u16,
    pub SetCapabilitiesStatus: u32,
}
pub type ACPI_RASF_SHARED_MEMORY = acpi_rasf_shared_memory;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_rasf_parameter_block {
    pub Type: u16,
    pub Version: u16,
    pub Length: u16,
}
pub type ACPI_RASF_PARAMETER_BLOCK = acpi_rasf_parameter_block;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_rasf_patrol_scrub_parameter {
    pub Header: ACPI_RASF_PARAMETER_BLOCK,
    pub PatrolScrubCommand: u16,
    pub RequestedAddressRange: [u64; 2usize],
    pub ActualAddressRange: [u64; 2usize],
    pub Flags: u16,
    pub RequestedSpeed: u8,
}
pub type ACPI_RASF_PATROL_SCRUB_PARAMETER = acpi_rasf_patrol_scrub_parameter;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiRasfCommands {
    ACPI_RASF_EXECUTE_RASF_COMMAND = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiRasfCapabiliities {
    ACPI_HW_PATROL_SCRUB_SUPPORTED = 0,
    ACPI_SW_PATROL_SCRUB_EXPOSED = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiRasfPatrolScrubCommands {
    ACPI_RASF_GET_PATROL_PARAMETERS = 1,
    ACPI_RASF_START_PATROL_SCRUBBER = 2,
    ACPI_RASF_STOP_PATROL_SCRUBBER = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiRasfStatus {
    ACPI_RASF_SUCCESS = 0,
    ACPI_RASF_NOT_VALID = 1,
    ACPI_RASF_NOT_SUPPORTED = 2,
    ACPI_RASF_BUSY = 3,
    ACPI_RASF_FAILED = 4,
    ACPI_RASF_ABORTED = 5,
    ACPI_RASF_INVALID_DATA = 6,
}