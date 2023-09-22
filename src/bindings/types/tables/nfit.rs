use crate::bindings::types::FfiAcpiTableHeader;

///  NFIT - NVDIMM Interface Table (ACPI 6.0+)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableNfit {
    pub Header: FfiAcpiTableHeader,
    pub Reserved: u32,
}
///  NFIT - NVDIMM Interface Table (ACPI 6.0+)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitHeader {
    pub Type: u16,
    pub Length: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiNfitType {
    ACPI_NFIT_TYPE_SYSTEM_ADDRESS = 0,
    ACPI_NFIT_TYPE_MEMORY_MAP = 1,
    ACPI_NFIT_TYPE_INTERLEAVE = 2,
    ACPI_NFIT_TYPE_SMBIOS = 3,
    ACPI_NFIT_TYPE_CONTROL_REGION = 4,
    ACPI_NFIT_TYPE_DATA_REGION = 5,
    ACPI_NFIT_TYPE_FLUSH_ADDRESS = 6,
    ACPI_NFIT_TYPE_CAPABILITIES = 7,
    ACPI_NFIT_TYPE_RESERVED = 8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitSystemAddress {
    pub Header: FfiAcpiNfitHeader,
    pub RangeIndex: u16,
    pub Flags: u16,
    pub Reserved: u32,
    pub ProximityDomain: u32,
    pub RangeGuid: [u8; 16usize],
    pub Address: u64,
    pub Length: u64,
    pub MemoryMapping: u64,
    pub LocationCookie: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitMemoryMap {
    pub Header: FfiAcpiNfitHeader,
    pub DeviceHandle: u32,
    pub PhysicalId: u16,
    pub RegionId: u16,
    pub RangeIndex: u16,
    pub RegionIndex: u16,
    pub RegionSize: u64,
    pub RegionOffset: u64,
    pub Address: u64,
    pub InterleaveIndex: u16,
    pub InterleaveWays: u16,
    pub Flags: u16,
    pub Reserved: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitInterleave {
    pub Header: FfiAcpiNfitHeader,
    pub InterleaveIndex: u16,
    pub Reserved: u16,
    pub LineCount: u32,
    pub LineSize: u32,
    pub LineOffset: [u32; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitSmbios {
    pub Header: FfiAcpiNfitHeader,
    pub Reserved: u32,
    pub Data: [u8; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitControlRegion {
    pub Header: FfiAcpiNfitHeader,
    pub RegionIndex: u16,
    pub VendorId: u16,
    pub DeviceId: u16,
    pub RevisionId: u16,
    pub SubsystemVendorId: u16,
    pub SubsystemDeviceId: u16,
    pub SubsystemRevisionId: u16,
    pub ValidFields: u8,
    pub ManufacturingLocation: u8,
    pub ManufacturingDate: u16,
    pub Reserved: [u8; 2usize],
    pub SerialNumber: u32,
    pub Code: u16,
    pub Windows: u16,
    pub WindowSize: u64,
    pub CommandOffset: u64,
    pub CommandSize: u64,
    pub StatusOffset: u64,
    pub StatusSize: u64,
    pub Flags: u16,
    pub Reserved1: [u8; 6usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitDataRegion {
    pub Header: FfiAcpiNfitHeader,
    pub RegionIndex: u16,
    pub Windows: u16,
    pub Offset: u64,
    pub Size: u64,
    pub Capacity: u64,
    pub StartAddress: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitFlushAddress {
    pub Header: FfiAcpiNfitHeader,
    pub DeviceHandle: u32,
    pub HintCount: u16,
    pub Reserved: [u8; 6usize],
    pub HintAddress: [u64; 1usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiNfitCapabilities {
    pub Header: FfiAcpiNfitHeader,
    pub HighestCapability: u8,
    pub Reserved: [u8; 3usize],
    pub Capabilities: u32,
    pub Reserved2: u32,
}

