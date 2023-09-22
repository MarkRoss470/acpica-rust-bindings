use crate::{interface::AcpiGenericAddress, types::AcpiTableHeader};

use super::{FfiAcpiSubtableHeader, FfiAcpiTableHeader};

///  PCCT - Platform Communications Channel Table (ACPI 5.0)
///         Version 2 (ACPI 6.2)
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePcct {
    pub header: FfiAcpiTableHeader,
    pub flags: u32,
    pub reserved: u64,
}

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
    pub header: FfiAcpiSubtableHeader,
    pub reserved: [u8; 6usize],
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: AcpiGenericAddress,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctHwReduced {
    pub header: FfiAcpiSubtableHeader,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved: u8,
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: AcpiGenericAddress,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctHwReducedType2 {
    pub header: FfiAcpiSubtableHeader,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved: u8,
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: AcpiGenericAddress,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
    pub platform_ack_register: AcpiGenericAddress,
    pub ack_preserve_mask: u64,
    pub ack_write_mask: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctExtPccMaster {
    pub header: FfiAcpiSubtableHeader,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved1: u8,
    pub base_address: u64,
    pub length: u32,
    pub doorbell_register: AcpiGenericAddress,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u32,
    pub platform_ack_register: AcpiGenericAddress,
    pub ack_preserve_mask: u64,
    pub ack_set_mask: u64,
    pub reserved2: u64,
    pub cmd_complete_register: AcpiGenericAddress,
    pub cmd_complete_mask: u64,
    pub cmd_update_register: AcpiGenericAddress,
    pub cmd_update_preserve_mask: u64,
    pub cmd_update_set_mask: u64,
    pub error_status_register: AcpiGenericAddress,
    pub error_status_mask: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctExtPccSlave {
    pub header: FfiAcpiSubtableHeader,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved1: u8,
    pub base_address: u64,
    pub length: u32,
    pub doorbell_register: AcpiGenericAddress,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u32,
    pub platform_ack_register: AcpiGenericAddress,
    pub ack_preserve_mask: u64,
    pub ack_set_mask: u64,
    pub reserved2: u64,
    pub cmd_complete_register: AcpiGenericAddress,
    pub cmd_complete_mask: u64,
    pub cmd_update_register: AcpiGenericAddress,
    pub cmd_update_preserve_mask: u64,
    pub cmd_update_set_mask: u64,
    pub error_status_register: AcpiGenericAddress,
    pub error_status_mask: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctHwReg {
    pub header: FfiAcpiSubtableHeader,
    pub version: u16,
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: AcpiGenericAddress,
    pub doorbell_preserve: u64,
    pub doorbell_write: u64,
    pub cmd_complete_register: AcpiGenericAddress,
    pub cmd_complete_mask: u64,
    pub error_status_register: AcpiGenericAddress,
    pub error_status_mask: u64,
    pub nominal_latency: u32,
    pub min_turnaround_time: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctSharedMemory {
    pub signature: u32,
    pub command: u16,
    pub status: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPcctExtPccSharedMemory {
    pub signature: u32,
    pub flags: u32,
    pub length: u32,
    pub command: u32,
}

