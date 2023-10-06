#[cfg(not(feature = "builtin_lock"))]
use crate::types::AcpiCpuFlags;
#[cfg(not(all(
    feature = "builtin_cache",
    feature = "builtin_lock",
    feature = "builtin_semaphore"
)))]
use core::ffi::c_void;

#[cfg(not(feature = "builtin_alloc"))]
use crate::types::AcpiAllocationError;

use alloc::boxed::Box;

use crate::{handler::AcpiHandler, status::AcpiError};

#[allow(clippy::type_complexity)]
pub struct DummyHandler<'a> {
    pub fn_initialize: Box<dyn Fn() -> Result<(), AcpiError>>,
    pub fn_terminate: Box<dyn Fn() -> Result<(), AcpiError>>,

    pub fn_predefined_override: Box<
        dyn Fn(
            &crate::types::AcpiPredefinedNames,
        ) -> Result<Option<alloc::string::String>, crate::status::AcpiError>,
    >,

    pub fn_table_override: Box<
        dyn Fn(
            &crate::types::AcpiTableHeader,
        )
            -> Result<Option<crate::types::AcpiTableHeader<'a>>, crate::status::AcpiError>,
    >,

    pub fn_physical_table_override: Box<
        dyn Fn(
            &crate::types::AcpiTableHeader,
        ) -> Result<
            Option<(crate::types::AcpiPhysicalAddress, u32)>,
            crate::status::AcpiError,
        >,
    >,

    pub fn_get_root_pointer: Box<dyn Fn() -> crate::types::AcpiPhysicalAddress>,
    pub fn_map_memory: Box<
        dyn Fn(
            crate::types::AcpiPhysicalAddress,
            usize,
        ) -> Result<*mut u8, crate::types::AcpiMappingError>,
    >,

    pub fn_unmap_memory: Box<dyn Fn(*mut u8, usize)>,

    pub fn_get_physical_address: Box<
        dyn Fn(
            *mut u8,
        )
            -> Result<Option<crate::types::AcpiPhysicalAddress>, crate::status::AcpiError>,
    >,

    pub fn_install_interrupt_handler: Box<
        dyn Fn(u32, crate::types::AcpiInterruptCallback) -> Result<(), crate::status::AcpiError>,
    >,

    pub fn_remove_interrupt_handler: Box<
        dyn Fn(u32, crate::types::AcpiInterruptCallback) -> Result<(), crate::status::AcpiError>,
    >,

    pub fn_get_thread_id: Box<dyn Fn() -> u64>,

    pub fn_execute:
        Box<dyn Fn(crate::types::AcpiThreadCallback) -> Result<(), crate::status::AcpiError>>,

    pub fn_printf: Box<dyn Fn(core::fmt::Arguments)>,

    #[cfg(not(feature = "builtin_cache"))]
    pub fn_create_cache: Box<dyn Fn(&str, u16, u16) -> Result<*mut c_void, AcpiError>>,

    #[cfg(not(feature = "builtin_cache"))]
    pub fn_delete_cache: Box<dyn Fn(*mut c_void) -> Result<(), AcpiAllocationError>>,

    #[cfg(not(feature = "builtin_cache"))]
    pub fn_purge_cache: Box<dyn Fn(*mut c_void)>,

    #[cfg(not(feature = "builtin_cache"))]
    pub fn_acquire_object: Box<dyn Fn(*mut c_void) -> Option<*mut u8>>,

    #[cfg(not(feature = "builtin_cache"))]
    pub fn_release_object: Box<dyn Fn(*mut c_void, *mut u8)>,

    #[cfg(not(feature = "builtin_lock"))]
    pub fn_create_lock: Box<dyn Fn() -> Result<*mut c_void, AcpiError>>,

    #[cfg(not(feature = "builtin_lock"))]
    pub fn_delete_lock: Box<dyn Fn(*mut c_void)>,

    #[cfg(not(feature = "builtin_lock"))]
    pub fn_acquire_lock: Box<dyn Fn(*mut c_void) -> AcpiCpuFlags>,

    #[cfg(not(feature = "builtin_lock"))]
    pub fn_release_lock: Box<dyn Fn(*mut c_void, AcpiCpuFlags)>,

    #[cfg(not(feature = "builtin_semaphore"))]
    pub fn_create_semaphore: Box<dyn Fn(u32, u32) -> Result<*mut c_void, AcpiError>>,

    #[cfg(not(feature = "builtin_semaphore"))]
    pub fn_delete_semaphore: Box<dyn Fn(*mut c_void) -> Result<(), AcpiError>>,

    #[cfg(not(feature = "builtin_semaphore"))]
    pub fn_wait_semaphore: Box<dyn Fn(*mut c_void, u32, u16) -> Result<(), AcpiError>>,

    #[cfg(not(feature = "builtin_semaphore"))]
    pub fn_signal_semaphore: Box<dyn Fn(*mut c_void, u32) -> Result<(), AcpiError>>,

    #[cfg(not(feature = "builtin_alloc"))]
    pub fn_allocate: Box<dyn Fn(usize) -> Result<*mut u8, AcpiAllocationError>>,

    #[cfg(not(feature = "builtin_alloc"))]
    pub fn_free: Box<dyn Fn(*mut u8)>,
}

// SAFETY: This struct is only used in tests, which are carried out in a no_std environment not linked to any OS
// Therefore all test code is single threaded and so this impl is sound
unsafe impl Send for DummyHandler<'static> {}

impl<'a> DummyHandler<'a> {
    pub(crate) fn new() -> Self {
        Self {
            fn_get_root_pointer: Box::new(|| panic!("Dummy function on test struct called")),
            fn_map_memory: Box::new(|_, _| panic!("Dummy function on test struct called")),
            fn_unmap_memory: Box::new(|_, _| panic!("Dummy function on test struct called")),
            fn_get_physical_address: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_install_interrupt_handler: Box::new(|_, _| {
                panic!("Dummy function on test struct called")
            }),
            fn_remove_interrupt_handler: Box::new(|_, _| {
                panic!("Dummy function on test struct called")
            }),
            fn_get_thread_id: Box::new(|| panic!("Dummy function on test struct called")),
            fn_execute: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_printf: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_initialize: Box::new(|| panic!("Dummy function on test struct called")),
            fn_terminate: Box::new(|| panic!("Dummy function on test struct called")),
            fn_predefined_override: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_table_override: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_physical_table_override: Box::new(|_| {
                panic!("Dummy function on test struct called")
            }),

            #[cfg(not(feature = "builtin_cache"))]
            fn_create_cache: Box::new(|_, _, _| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_cache"))]
            fn_delete_cache: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_cache"))]
            fn_purge_cache: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_cache"))]
            fn_acquire_object: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_cache"))]
            fn_release_object: Box::new(|_, _| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_lock"))]
            fn_create_lock: Box::new(|| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_lock"))]
            fn_delete_lock: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_lock"))]
            fn_acquire_lock: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_lock"))]
            fn_release_lock: Box::new(|_, _| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_semaphore"))]
            fn_create_semaphore: Box::new(|_, _| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_semaphore"))]
            fn_delete_semaphore: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_semaphore"))]
            fn_wait_semaphore: Box::new(|_, _, _| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_semaphore"))]
            fn_signal_semaphore: Box::new(|_, _| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_alloc"))]
            fn_allocate: Box::new(|_| panic!("Dummy function on test struct called")),
        
            #[cfg(not(feature = "builtin_alloc"))]
            fn_free: Box::new(|_| panic!("Dummy function on test struct called")),

        }
    }
}

// SAFETY: 
// Each method in this implementation is the user of the test struct's responsibility
unsafe impl<'a> AcpiHandler for DummyHandler<'a> {
    fn get_root_pointer(&mut self) -> crate::types::AcpiPhysicalAddress {
        (self.fn_get_root_pointer)()
    }

    unsafe fn map_memory(
        &mut self,
        physical_address: crate::types::AcpiPhysicalAddress,
        length: usize,
    ) -> Result<*mut u8, crate::types::AcpiMappingError> {
        (self.fn_map_memory)(physical_address, length)
    }

    unsafe fn unmap_memory(&mut self, address: *mut u8, length: usize) {
        (self.fn_unmap_memory)(address, length);
    }

    fn get_physical_address(
        &mut self,
        logical_address: *mut u8,
    ) -> Result<Option<crate::types::AcpiPhysicalAddress>, crate::status::AcpiError> {
        (self.fn_get_physical_address)(logical_address)
    }

    unsafe fn install_interrupt_handler(
        &mut self,
        interrupt_number: u32,
        callback: crate::types::AcpiInterruptCallback,
    ) -> Result<(), crate::status::AcpiError> {
        (self.fn_install_interrupt_handler)(interrupt_number, callback)
    }

    unsafe fn remove_interrupt_handler(
        &mut self,
        interrupt_number: u32,
        callback: crate::types::AcpiInterruptCallback,
    ) -> Result<(), crate::status::AcpiError> {
        (self.fn_remove_interrupt_handler)(interrupt_number, callback)
    }

    fn get_thread_id(&mut self) -> u64 {
        (self.fn_get_thread_id)()
    }

    unsafe fn execute(
        &mut self,
        // callback_type: AcpiExecuteType,
        callback: crate::types::AcpiThreadCallback,
    ) -> Result<(), crate::status::AcpiError> {
        (self.fn_execute)(callback)
    }

    fn printf(&mut self, message: core::fmt::Arguments) {
        (self.fn_printf)(message);
    }

    unsafe fn initialize(&mut self) -> Result<(), crate::status::AcpiError> {
        (self.fn_initialize)()
    }

    unsafe fn terminate(&mut self) -> Result<(), crate::status::AcpiError> {
        (self.fn_terminate)()
    }

    unsafe fn predefined_override(
        &mut self,
        predefined_object: &crate::types::AcpiPredefinedNames,
    ) -> Result<Option<alloc::string::String>, crate::status::AcpiError> {
        (self.fn_predefined_override)(predefined_object)
    }

    unsafe fn table_override(
        &mut self,
        table: &crate::types::AcpiTableHeader,
    ) -> Result<Option<crate::types::AcpiTableHeader>, crate::status::AcpiError> {
        (self.fn_table_override)(table)
    }

    unsafe fn physical_table_override(
        &mut self,
        table: &crate::types::AcpiTableHeader,
    ) -> Result<Option<(crate::types::AcpiPhysicalAddress, u32)>, crate::status::AcpiError> {
        (self.fn_physical_table_override)(table)
    }

    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn create_cache(
        &mut self,
        cache_name: &str,
        object_size: u16,
        max_depth: u16,
    ) -> Result<*mut c_void, AcpiError> {
        (self.fn_create_cache)(cache_name, object_size, max_depth)
    }

    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn delete_cache(&mut self, cache: *mut c_void) -> Result<(), AcpiAllocationError> {
        (self.fn_delete_cache)(cache)
    }

    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn purge_cache(&mut self, cache: *mut c_void) {
        (self.fn_purge_cache)(cache)
    }

    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn acquire_object(&mut self, cache: *mut c_void) -> Option<*mut u8> {
        (self.fn_acquire_object)(cache)
    }

    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn release_object(&mut self, cache: *mut c_void, object: *mut u8) {
        (self.fn_release_object)(cache, object)
    }

    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn create_lock(&mut self) -> Result<*mut c_void, AcpiError> {
        (self.fn_create_lock)()
    }

    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn delete_lock(&mut self, lock: *mut c_void) {
        (self.fn_delete_lock)(lock)
    }

    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn acquire_lock(&mut self, handle: *mut c_void) -> AcpiCpuFlags {
        (self.fn_acquire_lock)(handle)
    }

    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn release_lock(&mut self, handle: *mut c_void, flags: AcpiCpuFlags) {
        (self.fn_release_lock)(handle, flags)
    }

    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn create_semaphore(
        &mut self,
        max_units: u32,
        initial_units: u32,
    ) -> Result<*mut c_void, AcpiError> {
        (self.fn_create_semaphore)(max_units, initial_units)
    }

    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn delete_semaphore(&mut self, handle: *mut c_void) -> Result<(), AcpiError> {
        (self.fn_delete_semaphore)(handle)
    }

    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn wait_semaphore(
        &mut self,
        handle: *mut c_void,
        units: u32,
        timeout: u16,
    ) -> Result<(), AcpiError> {
        (self.fn_wait_semaphore)(handle, units, timeout)
    }

    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn signal_semaphore(
        &mut self,
        handle: *mut c_void,
        units: u32,
    ) -> Result<(), AcpiError> {
        (self.fn_signal_semaphore)(handle, units)
    }

    #[cfg(not(feature = "builtin_alloc"))]
    unsafe fn allocate(&mut self, size: usize) -> Result<*mut u8, AcpiAllocationError> {
        (self.fn_allocate)(size)
    }

    #[cfg(not(feature = "builtin_alloc"))]
    unsafe fn free(&mut self, memory: *mut u8) {
        (self.fn_free)(memory)
    }
}
