use crate::{interface::AcpiGenericAddress, types::AcpiTableHeader};

use super::{FfiAcpiSubtableHeader, FfiAcpiTableHeader};

///  PCCT - Platform Communications Channel Table (ACPI 5.0)
///         Version 2 (ACPI 6.2)
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePcct {
    pub Header: FfiAcpiTableHeader,
    pub Flags: u32,
    pub Reserved: u64,
}
///  PCCT - Platform Communications Channel Table (ACPI 5.0)
///         Version 2 (ACPI 6.2)
/// 

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiPcctType {
    ACPI_PCCT_TYPE_GENERIC_SUBSPACE = 0,
    ACPI_PCCT_TYPE_HW_REDUCED_SUBSPACE = 1,
    ACPI_PCCT_TYPE_HW_REDUCED_SUBSPACE_TYPE2 = 2,
    ACPI_PCCT_TYPE_EXT_PCC_MASTER_SUBSPACE = 3,
    ACPI_PCCT_TYPE_EXT_PCC_SLAVE_SUBSPACE = 4,
    ACPI_PCCT_TYPE_HW_REG_COMM_SUBSPACE = 5,
    ACPI_PCCT_TYPE_RESERVED = 6,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctSubspace {
    pub Header: FfiAcpiSubtableHeader,
    pub Reserved: [u8; 6usize],
    pub BaseAddress: u64,
    pub Length: u64,
    pub DoorbellRegister: AcpiGenericAddress,
    pub PreserveMask: u64,
    pub WriteMask: u64,
    pub Latency: u32,
    pub MaxAccessRate: u32,
    pub MinTurnaroundTime: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctHwReduced {
    pub Header: FfiAcpiSubtableHeader,
    pub PlatformInterrupt: u32,
    pub Flags: u8,
    pub Reserved: u8,
    pub BaseAddress: u64,
    pub Length: u64,
    pub DoorbellRegister: AcpiGenericAddress,
    pub PreserveMask: u64,
    pub WriteMask: u64,
    pub Latency: u32,
    pub MaxAccessRate: u32,
    pub MinTurnaroundTime: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctHwReducedType2 {
    pub Header: FfiAcpiSubtableHeader,
    pub PlatformInterrupt: u32,
    pub Flags: u8,
    pub Reserved: u8,
    pub BaseAddress: u64,
    pub Length: u64,
    pub DoorbellRegister: AcpiGenericAddress,
    pub PreserveMask: u64,
    pub WriteMask: u64,
    pub Latency: u32,
    pub MaxAccessRate: u32,
    pub MinTurnaroundTime: u16,
    pub PlatformAckRegister: AcpiGenericAddress,
    pub AckPreserveMask: u64,
    pub AckWriteMask: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctExtPccMaster {
    pub Header: FfiAcpiSubtableHeader,
    pub PlatformInterrupt: u32,
    pub Flags: u8,
    pub Reserved1: u8,
    pub BaseAddress: u64,
    pub Length: u32,
    pub DoorbellRegister: AcpiGenericAddress,
    pub PreserveMask: u64,
    pub WriteMask: u64,
    pub Latency: u32,
    pub MaxAccessRate: u32,
    pub MinTurnaroundTime: u32,
    pub PlatformAckRegister: AcpiGenericAddress,
    pub AckPreserveMask: u64,
    pub AckSetMask: u64,
    pub Reserved2: u64,
    pub CmdCompleteRegister: AcpiGenericAddress,
    pub CmdCompleteMask: u64,
    pub CmdUpdateRegister: AcpiGenericAddress,
    pub CmdUpdatePreserveMask: u64,
    pub CmdUpdateSetMask: u64,
    pub ErrorStatusRegister: AcpiGenericAddress,
    pub ErrorStatusMask: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctExtPccSlave {
    pub Header: FfiAcpiSubtableHeader,
    pub PlatformInterrupt: u32,
    pub Flags: u8,
    pub Reserved1: u8,
    pub BaseAddress: u64,
    pub Length: u32,
    pub DoorbellRegister: AcpiGenericAddress,
    pub PreserveMask: u64,
    pub WriteMask: u64,
    pub Latency: u32,
    pub MaxAccessRate: u32,
    pub MinTurnaroundTime: u32,
    pub PlatformAckRegister: AcpiGenericAddress,
    pub AckPreserveMask: u64,
    pub AckSetMask: u64,
    pub Reserved2: u64,
    pub CmdCompleteRegister: AcpiGenericAddress,
    pub CmdCompleteMask: u64,
    pub CmdUpdateRegister: AcpiGenericAddress,
    pub CmdUpdatePreserveMask: u64,
    pub CmdUpdateSetMask: u64,
    pub ErrorStatusRegister: AcpiGenericAddress,
    pub ErrorStatusMask: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctHwReg {
    pub Header: FfiAcpiSubtableHeader,
    pub Version: u16,
    pub BaseAddress: u64,
    pub Length: u64,
    pub DoorbellRegister: AcpiGenericAddress,
    pub DoorbellPreserve: u64,
    pub DoorbellWrite: u64,
    pub CmdCompleteRegister: AcpiGenericAddress,
    pub CmdCompleteMask: u64,
    pub ErrorStatusRegister: AcpiGenericAddress,
    pub ErrorStatusMask: u64,
    pub NominalLatency: u32,
    pub MinTurnaroundTime: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctSharedMemory {
    pub Signature: u32,
    pub Command: u16,
    pub Status: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctExtPccSharedMemory {
    pub Signature: u32,
    pub Flags: u32,
    pub Length: u32,
    pub Command: u32,
}

