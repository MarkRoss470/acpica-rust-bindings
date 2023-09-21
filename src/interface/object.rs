use crate::bindings::types::{
    object::{AcpiObjectType, FfiAcpiObject},
    FfiAcpiHandle, FfiAcpiIoAddress,
};

pub struct AcpiObjectPackage {
    count: u32,
    elements: *mut FfiAcpiObject,
}

pub enum AcpiObject {
    Integer(u64),
    String {
        length: u32,
        pointer: *mut u8,
    },
    Buffer {
        length: u32,
        pointer: *mut u8,
    },
    Package(AcpiObjectPackage),
    Reference {
        actual_type: AcpiObjectType,
        handle: FfiAcpiHandle,
    },
    Processor {
        proc_id: u32,
        pblk_address: FfiAcpiIoAddress,
        pblk_length: u32,
    },
    PowerResource {
        system_level: u32,
        resource_order: u32,
    },
}
