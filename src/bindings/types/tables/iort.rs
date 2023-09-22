use crate::bindings::types::FfiAcpiTableHeader;

///  IORT - IO Remapping Table
/// 
///  Conforms to \"IO Remapping Table System Software on ARM Platforms\",
///  Document number: ARM DEN 0049E.b, Feb 2021
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableIort {
    pub Header: FfiAcpiTableHeader,
    pub NodeCount: u32,
    pub NodeOffset: u32,
    pub Reserved: u32,
}
///  IORT - IO Remapping Table
/// 
///  Conforms to \"IO Remapping Table System Software on ARM Platforms\",
///  Document number: ARM DEN 0049E.b, Feb 2021
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortNode {
    pub Type: u8,
    pub Length: u16,
    pub Revision: u8,
    pub Identifier: u32,
    pub MappingCount: u32,
    pub MappingOffset: u32,
    pub NodeData: [i8; 1usize],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiIortNodeType {
    ACPI_IORT_NODE_ITS_GROUP = 0,
    ACPI_IORT_NODE_NAMED_COMPONENT = 1,
    ACPI_IORT_NODE_PCI_ROOT_COMPLEX = 2,
    ACPI_IORT_NODE_SMMU = 3,
    ACPI_IORT_NODE_SMMU_V3 = 4,
    ACPI_IORT_NODE_PMCG = 5,
    ACPI_IORT_NODE_RMR = 6,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortIdMapping {
    pub InputBase: u32,
    pub IdCount: u32,
    pub OutputBase: u32,
    pub OutputReference: u32,
    pub Flags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortMemoryAccess {
    pub CacheCoherency: u32,
    pub Hints: u8,
    pub Reserved: u16,
    pub MemoryFlags: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortItsGroup {
    pub ItsCount: u32,
    pub Identifiers: [u32; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortNamedComponent {
    pub NodeFlags: u32,
    pub MemoryProperties: u64,
    pub MemoryAddressLimit: u8,
    pub DeviceName: [i8; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortRootComplex {
    pub MemoryProperties: u64,
    pub AtsAttribute: u32,
    pub PciSegmentNumber: u32,
    pub MemoryAddressLimit: u8,
    pub Reserved: [u8; 3usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortSmmu {
    pub BaseAddress: u64,
    pub Span: u64,
    pub Model: u32,
    pub Flags: u32,
    pub GlobalInterruptOffset: u32,
    pub ContextInterruptCount: u32,
    pub ContextInterruptOffset: u32,
    pub PmuInterruptCount: u32,
    pub PmuInterruptOffset: u32,
    pub Interrupts: [u64; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortSmmuGsi {
    pub NSgIrpt: u32,
    pub NSgIrptFlags: u32,
    pub NSgCfgIrpt: u32,
    pub NSgCfgIrptFlags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortSmmuV3 {
    pub BaseAddress: u64,
    pub Flags: u32,
    pub Reserved: u32,
    pub VatosAddress: u64,
    pub Model: u32,
    pub EventGsiv: u32,
    pub PriGsiv: u32,
    pub GerrGsiv: u32,
    pub SyncGsiv: u32,
    pub Pxm: u32,
    pub IdMappingIndex: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortPmcg {
    pub Page0BaseAddress: u64,
    pub OverflowGsiv: u32,
    pub NodeReference: u32,
    pub Page1BaseAddress: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortRmr {
    pub Flags: u32,
    pub RmrCount: u32,
    pub RmrOffset: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIortRmrDesc {
    pub BaseAddress: u64,
    pub Length: u64,
    pub Reserved: u32,
}
