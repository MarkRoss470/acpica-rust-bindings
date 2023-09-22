use crate::bindings::types::FfiAcpiTableHeader;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_mpst {
    pub Header: FfiAcpiTableHeader,
    pub ChannelId: u8,
    pub Reserved1: [u8; 3usize],
    pub PowerNodeCount: u16,
    pub Reserved2: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_channel {
    pub ChannelId: u8,
    pub Reserved1: [u8; 3usize],
    pub PowerNodeCount: u16,
    pub Reserved2: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_power_node {
    pub Flags: u8,
    pub Reserved1: u8,
    pub NodeId: u16,
    pub Length: u32,
    pub RangeAddress: u64,
    pub RangeLength: u64,
    pub NumPowerStates: u32,
    pub NumPhysicalComponents: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_power_state {
    pub PowerState: u8,
    pub InfoIndex: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_component {
    pub ComponentId: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_data_hdr {
    pub CharacteristicsCount: u16,
    pub Reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_power_data {
    pub StructureId: u8,
    pub Flags: u8,
    pub Reserved1: u16,
    pub AveragePower: u32,
    pub PowerSaving: u32,
    pub ExitLatency: u64,
    pub Reserved2: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mpst_shared {
    pub Signature: u32,
    pub PccCommand: u16,
    pub PccStatus: u16,
    pub CommandRegister: u32,
    pub StatusRegister: u32,
    pub PowerStateId: u32,
    pub PowerNodeId: u32,
    pub EnergyConsumed: u64,
    pub AveragePower: u64,
}
