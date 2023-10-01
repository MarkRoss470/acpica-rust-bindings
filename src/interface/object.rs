//! The [`AcpiObject`] type

use core::ffi::CStr;

use log::debug;

use crate::bindings::{
    consts::{
        ACPI_TYPE_ANY, ACPI_TYPE_BUFFER, ACPI_TYPE_INTEGER, ACPI_TYPE_LOCAL_REFERENCE,
        ACPI_TYPE_PACKAGE, ACPI_TYPE_POWER, ACPI_TYPE_PROCESSOR, ACPI_TYPE_STRING,
    },
    types::{
        object::{FfiAcpiObject, FfiAcpiObjectType},
        FfiAcpiHandle, FfiAcpiIoAddress,
    },
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
pub enum AcpiObject<'a> {
    /// The object can be any type, or the type is not known.
    /// From ACPICA comments: "\[Any\] is used to indicate a NULL package element or an unresolved named reference."
    Any,
    /// The object is an integer
    Integer(u64),
    /// The object is a string
    String(&'a str),
    /// The object is a buffer of bytes
    Buffer(&'a [u8]),
    /// The object is a package containing other AML data
    Package(AcpiObjectPackage),
    /// The object is a reference to another [`AcpiObject`]
    Reference(AcpiObjectReference),
    /// The object describes the features of a processor
    Processor(AcpiObjectProcessor),
    /// The object describes a power resource
    PowerResource(AcpiObjectPowerResource),
}

impl<'a> AcpiObject<'a> {
    pub(crate) unsafe fn from_ffi(pointer: *const FfiAcpiObject) -> Self {
        let ffi_acpi_object = (*pointer);
        let object_type = ffi_acpi_object.object_type;

        match object_type {
            _t @ ACPI_TYPE_ANY => Self::Any,
            _t @ ACPI_TYPE_INTEGER => Self::Integer(ffi_acpi_object.integer.value),
            _t @ ACPI_TYPE_STRING => {
                let bytes = core::slice::from_raw_parts(
                    ffi_acpi_object.string.pointer.cast(),
                    ffi_acpi_object.string.length as _,
                );
                let str = core::str::from_utf8(bytes).unwrap();
                Self::String(str)
            }
            _t @ ACPI_TYPE_BUFFER => {
                let bytes = core::slice::from_raw_parts(
                    ffi_acpi_object.buffer.pointer,
                    ffi_acpi_object.buffer.length as _,
                );
                Self::Buffer(bytes)
            }
            _t @ ACPI_TYPE_PACKAGE => Self::Package(AcpiObjectPackage {
                count: ffi_acpi_object.package.count,
                elements: ffi_acpi_object.package.elements,
            }),
            _t @ ACPI_TYPE_LOCAL_REFERENCE => Self::Reference(AcpiObjectReference {
                actual_type: ffi_acpi_object.reference.actual_type,
                handle: ffi_acpi_object.reference.handle,
            }),
            _t @ ACPI_TYPE_PROCESSOR => Self::Processor(AcpiObjectProcessor {
                proc_id: ffi_acpi_object.processor.proc_id,
                pblk_address: ffi_acpi_object.processor.pblk_address,
                pblk_length: ffi_acpi_object.processor.pblk_length,
            }),
            _t @ ACPI_TYPE_POWER => Self::PowerResource(AcpiObjectPowerResource {
                system_level: ffi_acpi_object.power_resource.system_level,
                resource_order: ffi_acpi_object.power_resource.resource_order,
            }),

            _ => Self::Any,
        }
    }

    pub(crate) unsafe fn from_type_and_val(object_type: u8, val: *mut i8) -> Self {
        match object_type.into() {
            _t @ ACPI_TYPE_ANY => Self::Any,
            _t @ ACPI_TYPE_INTEGER => Self::Integer(val as _),
            _t @ ACPI_TYPE_STRING => {
                let s = CStr::from_ptr(val)
                    .to_str()
                    .expect("String object should have been valid utf-8");
                Self::String(s)
            }

            _ => {
                debug!(target: "acpi_object_from_type_and_val", "Check object type meaning. Type is {}, val is {:p}", object_type, val);
                Self::Any
            }
        }
    }
}
