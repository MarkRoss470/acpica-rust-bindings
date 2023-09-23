//! The [`AcpiObject`] type

use crate::bindings::types::{
    object::{FfiAcpiObject, FfiAcpiObjectType},
    FfiAcpiHandle, FfiAcpiIoAddress,
};

/// A package [`AcpiObject`]
#[derive(Debug)]
pub struct AcpiObjectPackage {
    count: u32,
    elements: *mut FfiAcpiObject,
}

/// A string [`AcpiObject`]
#[derive(Debug)]
pub struct AcpiObjectString {
    length: u32,
    pointer: *mut u8,
}

/// A buffer [`AcpiObject`]
#[derive(Debug)]
pub struct AcpiObjectBuffer {
    length: u32,
    pointer: *mut u8,
}

/// A reference to another [`AcpiObject`]
#[derive(Debug)]
pub struct AcpiObjectReference {
    actual_type: FfiAcpiObjectType,
    handle: FfiAcpiHandle,
}

/// An [`AcpiObject`] describing the features of a processor
#[derive(Debug)]
pub struct AcpiObjectProcessor {
    proc_id: u32,
    pblk_address: FfiAcpiIoAddress,
    pblk_length: u32,
}

/// An [`AcpiObject`] describing a power resource
#[derive(Debug)]
pub struct AcpiObjectPowerResource {
    system_level: u32,
    resource_order: u32,
}

/// An object used in the processing of ACPI data, mostly AML execution.
#[derive(Debug)]
pub enum AcpiObject {
    /// The object can be any type, or the type is not known.
    /// From ACPICA comments: "\[Any\] is used to indicate a NULL package element or an unresolved named reference."
    Any,
    /// The object is an integer
    Integer(u64),
    /// The object is a string
    String(AcpiObjectString),
    /// The object is a buffer of bytes
    Buffer(AcpiObjectBuffer),
    /// The object is a package containing other AML data
    Package(AcpiObjectPackage),
    /// The object is a reference to another [`AcpiObject`]
    Reference(AcpiObjectReference),
    /// The object describes the features of a processor
    Processor(AcpiObjectProcessor),
    /// The object describes a power resource
    PowerResource(AcpiObjectPowerResource),
}
