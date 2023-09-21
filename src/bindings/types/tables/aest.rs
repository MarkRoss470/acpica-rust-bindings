use crate::{types::AcpiTableHeader, bindings::types::__IncompleteArrayField};

///  AEST - Arm Error Source Table
/// 
///  Conforms to: ACPI for the Armv8 RAS Extensions 1.1 Platform Design Document
///  September 2020.
/// 
#[repr(C, packed)]
pub struct acpi_table_aest {
    pub Header: AcpiTableHeader,
    NodeArray: __IncompleteArrayField<*mut ::core::ffi::c_void>,
}
///  AEST - Arm Error Source Table
/// 
///  Conforms to: ACPI for the Armv8 RAS Extensions 1.1 Platform Design Document
///  September 2020.
/// 
pub type ACPI_TABLE_AEST = acpi_table_aest;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_hdr {
    pub Type: u8,
    pub Length: u16,
    pub Reserved: u8,
    pub NodeSpecificOffset: u32,
    pub NodeInterfaceOffset: u32,
    pub NodeInterruptOffset: u32,
    pub NodeInterruptCount: u32,
    pub TimestampRate: u64,
    pub Reserved1: u64,
    pub ErrorInjectionRate: u64,
}
pub type ACPI_AEST_HEADER = acpi_aest_hdr;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor {
    pub ProcessorId: u32,
    pub ResourceType: u8,
    pub Reserved: u8,
    pub Flags: u8,
    pub Revision: u8,
    pub ProcessorAffinity: u64,
}
pub type ACPI_AEST_PROCESSOR = acpi_aest_processor;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor_cache {
    pub CacheReference: u32,
    pub Reserved: u32,
}
pub type ACPI_AEST_PROCESSOR_CACHE = acpi_aest_processor_cache;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor_tlb {
    pub TlbLevel: u32,
    pub Reserved: u32,
}
pub type ACPI_AEST_PROCESSOR_TLB = acpi_aest_processor_tlb;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor_generic {
    pub Resource: *mut u8,
}
pub type ACPI_AEST_PROCESSOR_GENERIC = acpi_aest_processor_generic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_memory {
    pub SratProximityDomain: u32,
}
pub type ACPI_AEST_MEMORY = acpi_aest_memory;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_smmu {
    pub IortNodeReference: u32,
    pub SubcomponentReference: u32,
}
pub type ACPI_AEST_SMMU = acpi_aest_smmu;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_vendor {
    pub AcpiHid: u32,
    pub AcpiUid: u32,
    pub VendorSpecificData: [u8; 16usize],
}
pub type ACPI_AEST_VENDOR = acpi_aest_vendor;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_gic {
    pub InterfaceType: u32,
    pub InstanceId: u32,
}
pub type ACPI_AEST_GIC = acpi_aest_gic;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_node_interface {
    pub Type: u8,
    pub Reserved: [u8; 3usize],
    pub Flags: u32,
    pub Address: u64,
    pub ErrorRecordIndex: u32,
    pub ErrorRecordCount: u32,
    pub ErrorRecordImplemented: u64,
    pub ErrorStatusReporting: u64,
    pub AddressingMode: u64,
}
pub type ACPI_AEST_NODE_INTERFACE = acpi_aest_node_interface;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_node_interrupt {
    pub Type: u8,
    pub Reserved: [u8; 2usize],
    pub Flags: u8,
    pub Gsiv: u32,
    pub IortId: u8,
    pub Reserved1: [u8; 3usize],
}
pub type ACPI_AEST_NODE_INTERRUPT = acpi_aest_node_interrupt;