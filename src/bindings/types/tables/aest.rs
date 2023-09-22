use crate::bindings::types::{__IncompleteArrayField, FfiAcpiTableHeader};

///  AEST - Arm Error Source Table
/// 
///  Conforms to: ACPI for the Armv8 RAS Extensions 1.1 Platform Design Document
///  September 2020.
/// 
#[repr(C, packed)]
pub struct acpi_table_aest {
    pub Header: FfiAcpiTableHeader,
    NodeArray: __IncompleteArrayField<*mut ::core::ffi::c_void>,
}

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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor_cache {
    pub CacheReference: u32,
    pub Reserved: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor_tlb {
    pub TlbLevel: u32,
    pub Reserved: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_processor_generic {
    pub Resource: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_memory {
    pub SratProximityDomain: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_smmu {
    pub IortNodeReference: u32,
    pub SubcomponentReference: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_vendor {
    pub AcpiHid: u32,
    pub AcpiUid: u32,
    pub VendorSpecificData: [u8; 16usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_aest_gic {
    pub InterfaceType: u32,
    pub InstanceId: u32,
}
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