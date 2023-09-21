use core::{fmt::Debug, ffi::c_void};



#[repr(transparent)]
#[derive(Debug)]
pub struct AcpiPhysicalAddress(u64);

impl AcpiPhysicalAddress {
    pub const NULL: Self = Self(0);
}

#[derive(Debug)]
pub struct AcpiString(*mut u8);

impl AcpiString {
    pub const NULL: Self = Self(core::ptr::null_mut());
}

#[doc = " Master ACPI Table Header. This common header is used by all ACPI tables"]
#[doc = " except the RSDP and FACS."]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiTableHeader {
    pub Signature: [i8; 4usize],
    pub Length: u32,
    pub Revision: u8,
    pub Checksum: u8,
    pub OemId: [i8; 6usize],
    pub OemTableId: [i8; 8usize],
    pub OemRevision: u32,
    pub AslCompilerId: [i8; 4usize],
    pub AslCompilerRevision: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiPredefinedNames {
    pub Name: *const i8,
    pub Type: u8,
    pub Val: *mut i8,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiSize(usize);

impl AcpiSize {
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl Into<usize> for AcpiSize {
    fn into(self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiIoAddress(usize);

#[repr(transparent)]
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

#[repr(transparent)]
pub struct AcpiOsdExecCallback(Option<unsafe extern "C" fn(Context: *mut c_void)>);

#[repr(u32)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AcpiPciId {
    pub Segment: u16,
    pub Bus: u16,
    pub Device: u16,
    pub Function: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiTraceEventType {
    ACPI_TRACE_AML_METHOD = 0,
    ACPI_TRACE_AML_OPCODE = 1,
    ACPI_TRACE_AML_REGION = 2,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct AcpiSpinLock(*mut c_void);

#[repr(transparent)]
#[derive(Debug)]
pub struct AcpiSemaphore(*mut c_void);

#[derive(Debug)]
pub struct AcpiAllocationError;

#[repr(transparent)]
#[derive(Debug)]
pub struct AcpiCache(*mut u8);