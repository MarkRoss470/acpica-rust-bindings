//! Contains rust equivalents of types used by ACPICA

use core::{ffi::c_void, fmt::{Debug, Display}};

use crate::{
    bindings::types::{
        functions::{FfiAcpiOsdHandler, FfiAcpiOsdExecCallback}, tables::FfiAcpiTableHeader, FfiAcpiPredefinedNames,
    },
    interface::object::AcpiObject,
};

/// A physical address into main memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcpiPhysicalAddress(pub usize);

impl AcpiPhysicalAddress {
    /// A null pointer, i.e. a pointer of value 0
    pub const NULL: Self = Self(0);
}

///  Master ACPI Table Header. This common header is used by all ACPI tables
///  except the RSDP and FACS.
///
pub struct AcpiTableHeader<'a>(&'a mut FfiAcpiTableHeader);

impl<'a> AcpiTableHeader<'a> {
    pub(crate) fn from_ffi(ffi_header: &'a mut FfiAcpiTableHeader) -> Self {
        Self(ffi_header)
    }

    pub(crate) fn as_ffi(&mut self) -> &mut FfiAcpiTableHeader {
        self.0
    }
}

impl<'a> Debug for AcpiTableHeader<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AcpiTableHeader").field(&self.0).finish()
    }
}

/// A default value in an ACPI namespace.
///
/// This struct is passed to the [`predefined_override`] method to allow the OS to change the values of default objects.
///
/// [`predefined_override`]: crate::interface::handler::AcpiHandler::predefined_override
#[derive(Debug)]
pub struct AcpiPredefinedNames<'a>(&'a FfiAcpiPredefinedNames);

impl<'a> AcpiPredefinedNames<'a> {
    pub(crate) fn from_ffi(ffi_predefined_names: &'a FfiAcpiPredefinedNames) -> Self {
        Self(ffi_predefined_names)
    }

    pub(crate) fn as_ffi(&self) -> &'a FfiAcpiPredefinedNames {
        self.0
    }

    /// Gets the name of the object in the namespace
    pub fn name(&self) -> &str {
        todo!()
    }

    /// Gets the object which will be added to the namespace
    pub fn object(&self) -> AcpiObject {
        todo!()
    }
}

/// The address of an I/O port
#[derive(Debug, Clone, Copy)]
pub struct AcpiIoAddress(usize);

/// A callback to notify ACPICA about an interrupt.
///
/// The [`call`] method should be used to run the callback from the associated interrupt handler.
///
/// [`call`]: AcpiInterruptCallback::call
#[derive(Debug)]
pub struct AcpiInterruptCallback {
    function: FfiAcpiOsdHandler,
    context: *mut c_void,
}

/// Whether an interrupt is handled or not.
///
/// TODO: What should the OS use this for?
#[derive(Debug)]
pub enum AcpiInterruptHandledStatus {
    /// The interrupt has been handled
    Handled,
    /// The interrupt has not been handled
    NotHandled,
}

impl AcpiInterruptCallback {
    /// Calls the callback
    ///
    /// # Safety
    /// This method may only be called from the interrupt handler this callback is for.
    /// An interrupt vector will have been provided along with this object, and this method should only be called from the 
    /// interrupt handler for that interrupt vector.
    pub unsafe fn call(&mut self) -> AcpiInterruptHandledStatus {
        // SAFETY:
        let call_result = unsafe { (self.function)(self.context) };
        match call_result {
            0 => AcpiInterruptHandledStatus::Handled,
            1 => AcpiInterruptHandledStatus::NotHandled,
            _ => unreachable!("Acpi callback returned an invalid value"),
        }
    }
}

/// A callback to run in a new thread.
///
/// The [`call`] method should be used to run the callback from the created thread.
///
/// [`call`]: AcpiThreadCallback::call
#[derive(Debug)]
pub struct AcpiThreadCallback {
    function: FfiAcpiOsdExecCallback,
    context: *mut c_void,
}

impl AcpiThreadCallback {
    /// Calls the callback
    ///
    /// # Safety
    /// This method must be called from a new kernel thread.
    pub unsafe fn call(&mut self) {
        // SAFETY: This is 
        unsafe { (self.function)(self.context) };
    }
}

/// A PCI device, identified by its `segment:bus:device:function` address.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AcpiPciId {
    /// The PCI segment which the device is accessible through
    pub segment: u16,
    /// The PCI bus number
    pub bus: u16,
    /// The device number on the PCI bus
    pub device: u16,
    /// The function number of the device
    pub function: u16,
}

/// An error which can occur during allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiAllocationError {
    /// The system has run out of dynamic memory
    OutOfMemory,
}

impl Display for AcpiAllocationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Out of memory"),
        }
    }
}

/// An error which can occur when mapping physical memory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiMappingError {
    /// The system has run out of frames of memory to map
    OutOfMemory,
}

impl Display for AcpiMappingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Out of memory"),
        }
    }
}