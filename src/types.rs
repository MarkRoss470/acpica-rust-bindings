use core::{ffi::c_void, fmt::Debug};

use crate::{
    bindings::types::{tables::FfiAcpiTableHeader, FfiAcpiPhysicalAddress, FfiAcpiPredefinedNames},
    interface::object::AcpiObject,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcpiPhysicalAddress(usize);

impl AcpiPhysicalAddress {
    pub const NULL: Self = Self(0);

    /// Converts a [`usize`] to an [`AcpiPhysicalAddress`].
    pub const fn from_usize(v: usize) -> Self {
        Self(v)
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }

    pub const fn as_ffi(self) -> FfiAcpiPhysicalAddress {
        self.0
    }
}

///  Master ACPI Table Header. This common header is used by all ACPI tables
///  except the RSDP and FACS.
///
pub struct AcpiTableHeader<'a>(&'a mut FfiAcpiTableHeader);

impl<'a> AcpiTableHeader<'a> {
    pub(crate) fn from_ffi(ffi_header: &'a mut FfiAcpiTableHeader) -> Self {
        Self(ffi_header)
    }

    pub(crate) fn as_ffi(self) -> &'a mut FfiAcpiTableHeader {
        self.0
    }
}

impl<'a> Debug for AcpiTableHeader<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AcpiTableHeader").field(&self.0).finish()
    }
}

#[derive(Debug)]
pub struct AcpiPredefinedNames<'a>(&'a FfiAcpiPredefinedNames);

impl<'a> AcpiPredefinedNames<'a> {
    pub(crate) fn from_ffi(ffi_predefined_names: &'a FfiAcpiPredefinedNames) -> Self {
        Self(ffi_predefined_names)
    }

    pub(crate) fn as_ffi(&self) -> &'a FfiAcpiPredefinedNames {
        self.0
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn object_type(&self) -> AcpiObject {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AcpiSize(usize);

impl AcpiSize {
    pub fn from_usize(v: usize) -> Self {
        Self(v)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<AcpiSize> for usize {
    fn from(val: AcpiSize) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AcpiIoAddress(usize);

#[derive(Debug)]
pub(crate) struct AcpiOsdHandler(unsafe extern "C" fn(Context: *mut c_void) -> u32);

#[derive(Debug)]
pub struct AcpiCallback {
    function: AcpiOsdHandler,
    context: *mut c_void,
}

#[derive(Debug)]
pub enum AcpiInterruptHandledStatus {
    Handled,
    NotHandled,
}

impl AcpiCallback {
    pub unsafe fn call(&mut self) -> AcpiInterruptHandledStatus {
        let call_result = self.function.0(self.context);
        match call_result {
            0 => AcpiInterruptHandledStatus::Handled,
            1 => AcpiInterruptHandledStatus::NotHandled,
            _ => panic!("Acpi callback returned an invalid value"),
        }
    }
}

pub struct AcpiOsdExecCallback(Option<unsafe extern "C" fn(Context: *mut c_void)>);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiExecuteType {
    OSL_GLOBAL_LOCK_HANDLER = 0,
    OSL_NOTIFY_HANDLER = 1,
    OSL_GPE_HANDLER = 2,
    OSL_DEBUGGER_MAIN_THREAD = 3,
    OSL_DEBUGGER_EXEC_THREAD = 4,
    OSL_EC_POLL_HANDLER = 5,
    OSL_EC_BURST_HANDLER = 6,
}

#[derive(Debug, Copy, Clone)]
pub struct AcpiPciId {
    pub Segment: u16,
    pub Bus: u16,
    pub Device: u16,
    pub Function: u16,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiTraceEventType {
    ACPI_TRACE_AML_METHOD = 0,
    ACPI_TRACE_AML_OPCODE = 1,
    ACPI_TRACE_AML_REGION = 2,
}

#[derive(Debug)]
pub struct AcpiAllocationError;
