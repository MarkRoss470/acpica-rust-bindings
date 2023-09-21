use crate::{interface::AcpiGenericAddress, types::AcpiTableHeader};

use super::ACPI_SUBTABLE_HEADER;

#[doc = " PCCT - Platform Communications Channel Table (ACPI 5.0)"]
#[doc = "        Version 2 (ACPI 6.2)"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_pcct {
    pub Header: AcpiTableHeader,
    pub Flags: u32,
    pub Reserved: u64,
}
#[doc = " PCCT - Platform Communications Channel Table (ACPI 5.0)"]
#[doc = "        Version 2 (ACPI 6.2)"]
#[doc = ""]
pub type ACPI_TABLE_PCCT = acpi_table_pcct;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiPcctType {
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
pub struct acpi_pcct_subspace {
    pub Header: ACPI_SUBTABLE_HEADER,
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
pub type ACPI_PCCT_SUBSPACE = acpi_pcct_subspace;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_hw_reduced {
    pub Header: ACPI_SUBTABLE_HEADER,
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
pub type ACPI_PCCT_HW_REDUCED = acpi_pcct_hw_reduced;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_hw_reduced_type2 {
    pub Header: ACPI_SUBTABLE_HEADER,
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
pub type ACPI_PCCT_HW_REDUCED_TYPE2 = acpi_pcct_hw_reduced_type2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_ext_pcc_master {
    pub Header: ACPI_SUBTABLE_HEADER,
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
pub type ACPI_PCCT_EXT_PCC_MASTER = acpi_pcct_ext_pcc_master;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_ext_pcc_slave {
    pub Header: ACPI_SUBTABLE_HEADER,
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
pub type ACPI_PCCT_EXT_PCC_SLAVE = acpi_pcct_ext_pcc_slave;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_hw_reg {
    pub Header: ACPI_SUBTABLE_HEADER,
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
pub type ACPI_PCCT_HW_REG = acpi_pcct_hw_reg;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_shared_memory {
    pub Signature: u32,
    pub Command: u16,
    pub Status: u16,
}
pub type ACPI_PCCT_SHARED_MEMORY = acpi_pcct_shared_memory;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pcct_ext_pcc_shared_memory {
    pub Signature: u32,
    pub Flags: u32,
    pub Length: u32,
    pub Command: u32,
}
pub type ACPI_PCCT_EXT_PCC_SHARED_MEMORY = acpi_pcct_ext_pcc_shared_memory;
