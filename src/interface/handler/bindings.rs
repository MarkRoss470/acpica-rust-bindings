#[cfg(feature = "builtin_alloc")]
mod alloc;
#[cfg(feature = "builtin_lock")]
mod lock;
mod overrides;
mod printf;
#[cfg(feature = "builtin_semaphore")]
mod semaphore;

use crate::{
    bindings::types::{
        functions::{FfiAcpiOsdExecCallback, FfiAcpiOsdHandler},
        tables::FfiAcpiTableHeader,
        FfiAcpiExecuteType, FfiAcpiIoAddress, FfiAcpiPciId, FfiAcpiPhysicalAddress, FfiAcpiSize,
    },
    interface::status::{AcpiErrorAsStatusExt, AcpiStatus},
    types::AcpiPhysicalAddress,
};

use super::OS_INTERFACE;

#[export_name = "AcpiOsInitialize"]
extern "C" fn acpi_os_initialize() -> AcpiStatus {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();
    unsafe { interface.initialize().to_acpi_status() }
}

#[export_name = "AcpiOsTerminate"]
extern "C" fn acpi_os_terminate() -> AcpiStatus {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();
    unsafe { interface.terminate().to_acpi_status() }
}

#[export_name = "AcpiOsGetRootPointer"]
extern "C" fn acpi_os_get_root_pointer() -> FfiAcpiPhysicalAddress {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    interface.get_root_pointer().0
}

#[export_name = "AcpiOsMapMemory"]
extern "C" fn acpi_os_map_memory(
    address: FfiAcpiPhysicalAddress,
    length: FfiAcpiSize,
) -> *mut ::core::ffi::c_void {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe {
        interface
            .map_memory(AcpiPhysicalAddress(address), length)
            .map(<*mut u8>::cast)
            .unwrap_or(core::ptr::null_mut())
    }
}

#[export_name = "AcpiOsUnmapMemory"]
extern "C" fn acpi_os_unmap_memory(logical_address: *mut ::core::ffi::c_void, size: FfiAcpiSize) {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe { interface.unmap_memory(logical_address.cast(), size) }
}

#[export_name = "AcpiOsGetPhysicalAddress"]
extern "C" fn acpi_os_get_physical_address(
    _logical_address: *mut ::core::ffi::c_void,
    _physical_address: *mut FfiAcpiPhysicalAddress,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsInstallInterruptHandler"]
extern "C" fn acpi_os_install_interrupt_handler(
    _interrupt_number: u32,
    _service_routine: FfiAcpiOsdHandler,
    _context: *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsRemoveInterruptHandler"]
extern "C" fn acpi_os_remove_interrupt_handler(
    _interrupt_number: u32,
    _service_routine: FfiAcpiOsdHandler,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsGetThreadId"]
extern "C" fn acpi_os_get_thread_id() -> u64 {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    interface.get_thread_id()
}

#[export_name = "AcpiOsExecute"]
extern "C" fn acpi_os_execute(
    _type: FfiAcpiExecuteType,
    _function: FfiAcpiOsdExecCallback,
    _context: *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWaitEventsComplete"]
extern "C" fn acpi_os_wait_events_complete() {
    todo!()
}

#[export_name = "AcpiOsSleep"]
extern "C" fn acpi_os_sleep(_milliseconds: u64) {
    todo!()
}

#[export_name = "AcpiOsStall"]
extern "C" fn acpi_os_stall(_microseconds: u32) {
    todo!()
}

#[export_name = "AcpiOsReadPort"]
extern "C" fn acpi_os_read_port(
    _address: FfiAcpiIoAddress,
    _value: *mut u32,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWritePort"]
extern "C" fn acpi_os_write_port(
    _address: FfiAcpiIoAddress,
    _value: u32,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsReadMemory"]
extern "C" fn acpi_os_read_memory(
    _address: FfiAcpiPhysicalAddress,
    _value: *mut u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWriteMemory"]
extern "C" fn acpi_os_write_memory(
    _address: FfiAcpiPhysicalAddress,
    _value: u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsReadPciConfiguration"]
extern "C" fn acpi_os_read_pci_configuration(
    _pci_id: *mut FfiAcpiPciId,
    _reg: u32,
    _value: *mut u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWritePciConfiguration"]
extern "C" fn acpi_os_write_pci_configuration(
    _pci_id: *mut FfiAcpiPciId,
    _reg: u32,
    _value: u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsReadable"]
extern "C" fn acpi_os_readable(_pointer: *mut ::core::ffi::c_void, _length: FfiAcpiSize) -> bool {
    todo!()
}

#[export_name = "AcpiOsWritable"]
extern "C" fn acpi_os_writable(_pointer: *mut ::core::ffi::c_void, _length: FfiAcpiSize) -> bool {
    todo!()
}

#[export_name = "AcpiOsGetTimer"]
extern "C" fn acpi_os_get_timer() -> u64 {
    todo!()
}

#[export_name = "AcpiOsSignal"]
extern "C" fn acpi_os_signal(_function: u32, _info: *mut ::core::ffi::c_void) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsEnterSleep"]
extern "C" fn acpi_os_enter_sleep(
    _sleep_state: u8,
    _rega_value: u32,
    _regb_value: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsRedirectOutput"]
extern "C" fn acpi_os_redirect_output(_destination: *mut ::core::ffi::c_void) {
    todo!()
}

#[export_name = "AcpiOsGetLine"]
extern "C" fn acpi_os_get_line(
    _buffer: *mut i8,
    _buffer_length: u32,
    _bytes_read: *mut u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsInitializeDebugger"]
extern "C" fn acpi_os_initialize_debugger() -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsTerminateDebugger"]
extern "C" fn acpi_os_terminate_debugger() {
    todo!()
}

#[export_name = "AcpiOsWaitCommandReady"]
extern "C" fn acpi_os_wait_command_ready() -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsNotifyCommandComplete"]
extern "C" fn acpi_os_notify_command_complete() -> AcpiStatus {
    todo!()
}

// #[export_name = "AcpiOsTracePoint"]
// extern "C" fn acpi_os_trace_point(
//     _type: FfiAcpiTraceEventType,
//     _begin: bool,
//     _aml: *mut u8,
//     _pathname: *mut i8,
// ) {
//     todo!()
// }

#[export_name = "AcpiOsGetTableByName"]
extern "C" fn acpi_os_get_table_by_name(
    _signature: *mut i8,
    _instance: u32,
    _table: *mut *mut FfiAcpiTableHeader,
    _address: *mut FfiAcpiPhysicalAddress,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsGetTableByIndex"]
extern "C" fn acpi_os_get_table_by_index(
    _index: u32,
    _table: *mut *mut FfiAcpiTableHeader,
    _instance: *mut u32,
    _address: *mut FfiAcpiPhysicalAddress,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsGetTableByAddress"]
extern "C" fn acpi_os_get_table_by_address(
    _address: FfiAcpiPhysicalAddress,
    _table: *mut *mut FfiAcpiTableHeader,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsOpenDirectory"]
extern "C" fn acpi_os_open_directory(
    _pathname: *mut i8,
    _wildcard_spec: *mut i8,
    _requested_file_type: i8,
) -> *mut ::core::ffi::c_void {
    todo!()
}

#[export_name = "AcpiOsGetNextFilename"]
extern "C" fn acpi_os_get_next_filename(_dir_handle: *mut ::core::ffi::c_void) -> *mut i8 {
    todo!()
}

#[export_name = "AcpiOsCloseDirectory"]
extern "C" fn acpi_os_close_directory(_dir_handle: *mut ::core::ffi::c_void) {
    todo!()
}

/*


    Trait handlers for optional methods
    These handlers are only used if certain features are disabled.
    If the features are enabled, implementations provided by this crate are used instead of OS-specific implementations.


*/

// #[cfg(not(feature = "builtin_cache"))]

#[cfg(not(feature = "builtin_cache"))]
#[export_name = "AcpiOsCreateCache"]
extern "C" fn acpi_os_create_cache(
    cache_name: *mut i8,
    size: u16,
    max_depth: u16,
    return_cache: *mut *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_cache"))]
#[export_name = "AcpiOsDeleteCache"]
extern "C" fn acpi_os_delete_cache(cache: *mut ::core::ffi::c_void) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_cache"))]
#[export_name = "AcpiOsPurgeCache"]
extern "C" fn acpi_os_purge_cache(mut cache: *mut ::core::ffi::c_void) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_cache"))]
#[export_name = "AcpiOsAcquireObject"]
extern "C" fn acpi_os_acquire_object(
    mut cache: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    todo!()
}

#[cfg(not(feature = "builtin_cache"))]
#[export_name = "AcpiOsReleaseObject"]
extern "C" fn acpi_os_release_object(
    mut cache: *mut ::core::ffi::c_void,
    object: *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_alloc"))]
#[export_name = "AcpiOsAllocate"]
extern "C" fn acpi_os_allocate(size: FfiAcpiSize) -> *mut ::core::ffi::c_void {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe {
        interface
            .allocate(size)
            .unwrap_or(core::ptr::null_mut())
            .cast()
    }
}

// TODO: allow native allocate zeroed (set USE_NATIVE_ALLOCATE_ZEROED build flag)
// #[cfg(not(feature = "builtin_alloc"))]
// #[export_name = "AcpiOsAllocateZeroed"]
// extern "C" fn acpi_os_allocate_zeroed(_size: FfiAcpiSize) -> *mut ::core::ffi::c_void {
//     todo!()
// }

#[cfg(not(feature = "builtin_alloc"))]
#[export_name = "AcpiOsFree"]
extern "C" fn acpi_os_free(memory: *mut ::core::ffi::c_void) {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe { interface.free(memory.cast()) }
}

#[cfg(not(feature = "builtin_lock"))]
#[export_name = "AcpiOsCreateLock"]
extern "C" fn acpi_os_create_lock(out_handle: *mut *mut ::core::ffi::c_void) -> AcpiStatus {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    let created_lock = unsafe { interface.create_lock() };
    match created_lock {
        Ok(l) => unsafe {
            *out_handle = l;
            AcpiStatus::OK
        },
        Err(e) => e.to_acpi_status(),
    }
}

#[cfg(not(feature = "builtin_lock"))]
#[export_name = "AcpiOsDeleteLock"]
extern "C" fn acpi_os_delete_lock(handle: *mut ::core::ffi::c_void) {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe { interface.delete_lock(handle) }
}

#[cfg(not(feature = "builtin_lock"))]
use crate::{bindings::types::FfiAcpiCpuFlags, types::AcpiCpuFlags};

#[cfg(not(feature = "builtin_lock"))]
#[export_name = "AcpiOsAcquireLock"]
extern "C" fn acpi_os_acquire_lock(handle: *mut ::core::ffi::c_void) -> FfiAcpiCpuFlags {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe { interface.acquire_lock(handle).0 }
}

#[cfg(not(feature = "builtin_lock"))]
#[export_name = "AcpiOsReleaseLock"]
extern "C" fn acpi_os_release_lock(handle: *mut ::core::ffi::c_void, flags: FfiAcpiCpuFlags) {
    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    unsafe { interface.release_lock(handle, AcpiCpuFlags(flags)) }
}

#[cfg(not(feature = "builtin_semaphore"))]
#[export_name = "AcpiOsCreateSemaphore"]
extern "C" fn acpi_os_create_semaphore(
    _max_units: u32,
    _initial_units: u32,
    _out_handle: *mut *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_semaphore"))]
#[export_name = "AcpiOsDeleteSemaphore"]
extern "C" fn acpi_os_delete_semaphore(_handle: *mut ::core::ffi::c_void) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_semaphore"))]
#[export_name = "AcpiOsWaitSemaphore"]
extern "C" fn acpi_os_wait_semaphore(
    _handle: *mut ::core::ffi::c_void,
    _units: u32,
    _timeout: u16,
) -> AcpiStatus {
    todo!()
}

#[cfg(not(feature = "builtin_semaphore"))]
#[export_name = "AcpiOsSignalSemaphore"]
extern "C" fn acpi_os_signal_semaphore(
    _handle: *mut ::core::ffi::c_void,
    _units: u32,
) -> AcpiStatus {
    todo!()
}
