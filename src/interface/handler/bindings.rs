use crate::{
    interface::status::{AcpiErrorAsStatusExt, AcpiStatus},
    types::{
        AcpiExecuteType, AcpiIoAddress, AcpiOsdExecCallback, AcpiOsdHandler, AcpiPciId,
        AcpiPhysicalAddress, AcpiPredefinedNames, AcpiSemaphore, AcpiSize, AcpiSpinLock,
        AcpiString, AcpiTableHeader, AcpiTraceEventType,
    },
};

macro_rules! interface {
    () => {{
        use super::OS_INTERFACE;
        OS_INTERFACE.lock().as_mut().unwrap().as_mut()
    }};
}

#[export_name = "AcpiOsInitialize"]
extern "C" fn acpi_os_initialize() -> AcpiStatus {
    unsafe { interface!().initialize().as_acpi_status() }
}

#[export_name = "AcpiOsTerminate"]
extern "C" fn acpi_os_terminate() -> AcpiStatus {
    unsafe { interface!().terminate().as_acpi_status() }
}

#[export_name = "AcpiOsGetRootPointer"]
extern "C" fn acpi_os_get_root_pointer() -> AcpiPhysicalAddress {
    interface!().get_root_pointer()
}

#[export_name = "AcpiOsPredefinedOverride"]
extern "C" fn acpi_os_predefined_override(
    init_val: *const AcpiPredefinedNames,
    new_val_ptr: *mut AcpiString,
) -> AcpiStatus {
    // SAFETY: `init_val` is valid for reads
    let init_val = unsafe { &*init_val };

    // SAFETY: This is `AcpiOsPredefinedOverride`
    let result = unsafe { interface!().predefined_override(init_val) };
    let new_val = match result {
        Ok(new_val) => new_val,
        Err(e) => return e.as_acpi_status(),
    };

    // SAFETY: `new_val_ptr` is valid for writes
    unsafe {
        core::ptr::write_unaligned(new_val_ptr, new_val.unwrap_or(AcpiString::NULL));
    }

    AcpiStatus::OK
}

#[export_name = "AcpiOsTableOverride"]
extern "C" fn acpi_os_table_override(
    existing_table: *mut AcpiTableHeader,
    new_table_ptr: *mut *mut AcpiTableHeader,
) -> AcpiStatus {
    // SAFETY: `existing_table` is valid for reads
    let existing_table = unsafe { &*existing_table };

    // SAFETY: This is `AcpiOsTableOverride`
    let result = unsafe { interface!().table_override(existing_table) };
    let new_table = match result {
        Ok(new_table) => new_table,
        Err(e) => return e.as_acpi_status(),
    };

    // SAFETY: `new_table_ptr` is valid for writes
    unsafe {
        core::ptr::write_unaligned(
            new_table_ptr,
            new_table
                .map(|new_table| new_table as *mut _)
                .unwrap_or(core::ptr::null_mut()),
        );
    }

    AcpiStatus::OK
}

#[export_name = "AcpiOsPhysicalTableOverride"]
extern "C" fn acpi_os_physical_table_override(
    existing_table: *mut AcpiTableHeader,
    new_table_address_ptr: *mut AcpiPhysicalAddress,
    new_table_length_ptr: *mut u32,
) -> AcpiStatus {
    // SAFETY: `existing_table` is valid for reads
    let existing_table = unsafe { &*existing_table };

    // SAFETY: This is `AcpiOsPhysicalTableOverride`
    let result = unsafe { interface!().physical_table_override(existing_table) };
    let new_table = match result {
        Ok(new_table) => new_table,
        Err(e) => return e.as_acpi_status(),
    };

    match new_table {
        // SAFETY: `new_table_address_ptr` and `new_table_length_ptr` are valid for writes
        Some((address, length)) => unsafe {
            core::ptr::write(new_table_address_ptr, address);
            core::ptr::write(new_table_length_ptr, length);
        },
        // SAFETY: `new_table_address_ptr` is valid for writes
        None => unsafe {
            core::ptr::write(new_table_address_ptr, AcpiPhysicalAddress::NULL);
        },
    }

    AcpiStatus::OK
}

#[export_name = "AcpiOsCreateLock"]
extern "C" fn acpi_os_create_lock(_out_handle: *mut AcpiSpinLock) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsDeleteLock"]
extern "C" fn acpi_os_delete_lock(_handle: AcpiSpinLock) {
    todo!()
}

#[export_name = "AcpiOsAcquireLock"]
extern "C" fn acpi_os_acquire_lock(_handle: AcpiSpinLock) -> AcpiSize {
    todo!()
}

#[export_name = "AcpiOsReleaseLock"]
extern "C" fn acpi_os_release_lock(_handle: AcpiSpinLock, _flags: AcpiSize) {
    todo!()
}

#[export_name = "AcpiOsCreateSemaphore"]
extern "C" fn acpi_os_create_semaphore(
    _max_units: u32,
    _initial_units: u32,
    _out_handle: *mut AcpiSemaphore,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsDeleteSemaphore"]
extern "C" fn acpi_os_delete_semaphore(_handle: AcpiSemaphore) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWaitSemaphore"]
extern "C" fn acpi_os_wait_semaphore(
    _handle: AcpiSemaphore,
    _units: u32,
    _timeout: u16,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsSignalSemaphore"]
extern "C" fn acpi_os_signal_semaphore(_handle: AcpiSemaphore, _units: u32) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsAllocate"]
extern "C" fn acpi_os_allocate(_size: AcpiSize) -> *mut ::core::ffi::c_void {
    todo!()
}

// TODO: allow native allocate zeroed (set USE_NATIVE_ALLOCATE_ZEROED build flag)
// #[export_name = "AcpiOsAllocateZeroed"]
// extern "C" fn acpi_os_allocate_zeroed(_size: AcpiSize) -> *mut ::core::ffi::c_void {
//     todo!()
// }

#[export_name = "AcpiOsFree"]
extern "C" fn acpi_os_free(_memory: *mut ::core::ffi::c_void) {
    todo!()
}

#[export_name = "AcpiOsMapMemory"]
extern "C" fn acpi_os_map_memory(
    _where: AcpiPhysicalAddress,
    _length: AcpiSize,
) -> *mut ::core::ffi::c_void {
    todo!()
}

#[export_name = "AcpiOsUnmapMemory"]
extern "C" fn acpi_os_unmap_memory(_logical_address: *mut ::core::ffi::c_void, _size: AcpiSize) {
    todo!()
}

#[export_name = "AcpiOsGetPhysicalAddress"]
extern "C" fn acpi_os_get_physical_address(
    _logical_address: *mut ::core::ffi::c_void,
    _physical_address: *mut AcpiPhysicalAddress,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsCreateCache"]
extern "C" fn acpi_os_create_cache(
    _cache_name: *mut i8,
    _object_size: u16,
    _max_depth: u16,
    _return_cache: *mut *mut *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsDeleteCache"]
extern "C" fn acpi_os_delete_cache(_cache: *mut *mut ::core::ffi::c_void) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsPurgeCache"]
extern "C" fn acpi_os_purge_cache(_cache: *mut *mut ::core::ffi::c_void) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsAcquireObject"]
extern "C" fn acpi_os_acquire_object(
    _cache: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    todo!()
}

#[export_name = "AcpiOsReleaseObject"]
extern "C" fn acpi_os_release_object(
    _cache: *mut *mut ::core::ffi::c_void,
    _object: *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsInstallInterruptHandler"]
extern "C" fn acpi_os_install_interrupt_handler(
    _interrupt_number: u32,
    _service_routine: AcpiOsdHandler,
    _context: *mut ::core::ffi::c_void,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsRemoveInterruptHandler"]
extern "C" fn acpi_os_remove_interrupt_handler(
    _interrupt_number: u32,
    _service_routine: AcpiOsdHandler,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsGetThreadId"]
extern "C" fn acpi_os_get_thread_id() -> u64 {
    todo!()
}

#[export_name = "AcpiOsExecute"]
extern "C" fn acpi_os_execute(
    _type: AcpiExecuteType,
    _function: AcpiOsdExecCallback,
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
    _address: AcpiIoAddress,
    _value: *mut u32,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWritePort"]
extern "C" fn acpi_os_write_port(_address: AcpiIoAddress, _value: u32, _width: u32) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsReadMemory"]
extern "C" fn acpi_os_read_memory(
    _address: AcpiPhysicalAddress,
    _value: *mut u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWriteMemory"]
extern "C" fn acpi_os_write_memory(
    _address: AcpiPhysicalAddress,
    _value: u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsReadPciConfiguration"]
extern "C" fn acpi_os_read_pci_configuration(
    _pci_id: *mut AcpiPciId,
    _reg: u32,
    _value: *mut u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsWritePciConfiguration"]
extern "C" fn acpi_os_write_pci_configuration(
    _pci_id: *mut AcpiPciId,
    _reg: u32,
    _value: u64,
    _width: u32,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsReadable"]
extern "C" fn acpi_os_readable(_pointer: *mut ::core::ffi::c_void, _length: AcpiSize) -> bool {
    todo!()
}

#[export_name = "AcpiOsWritable"]
extern "C" fn acpi_os_writable(_pointer: *mut ::core::ffi::c_void, _length: AcpiSize) -> bool {
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

#[export_name = "AcpiOsPrintf"]
unsafe extern "C" fn acpi_os_printf(_format: *const i8, _args: ...) {
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

#[export_name = "AcpiOsTracePoint"]
extern "C" fn acpi_os_trace_point(
    _type: AcpiTraceEventType,
    _begin: bool,
    _aml: *mut u8,
    _pathname: *mut i8,
) {
    todo!()
}

#[export_name = "AcpiOsGetTableByName"]
extern "C" fn acpi_os_get_table_by_name(
    _signature: *mut i8,
    _instance: u32,
    _table: *mut *mut AcpiTableHeader,
    _address: *mut AcpiPhysicalAddress,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsGetTableByIndex"]
extern "C" fn acpi_os_get_table_by_index(
    _index: u32,
    _table: *mut *mut AcpiTableHeader,
    _instance: *mut u32,
    _address: *mut AcpiPhysicalAddress,
) -> AcpiStatus {
    todo!()
}

#[export_name = "AcpiOsGetTableByAddress"]
extern "C" fn acpi_os_get_table_by_address(
    _address: AcpiPhysicalAddress,
    _table: *mut *mut AcpiTableHeader,
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

#[export_name = "AcpiOsVprintf"]
unsafe extern "C" fn acpi_os_v_printf(_format: *const u8, _args: ...) {
    todo!()
}
