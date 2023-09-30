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
        ) -> Result<Option<crate::types::AcpiTableHeader<'a>>, crate::status::AcpiError>,
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
            fn_install_interrupt_handler: Box::new(|_, _| panic!("Dummy function on test struct called")),
            fn_remove_interrupt_handler: Box::new(|_, _| panic!("Dummy function on test struct called")),
            fn_get_thread_id: Box::new(|| panic!("Dummy function on test struct called")),
            fn_execute: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_printf: Box::new(|_| panic!("Dummy function on test struct called")),
            fn_initialize: Box::new(||panic!("Dummy function on test struct called")),
            fn_terminate: Box::new(||panic!("Dummy function on test struct called")),
            fn_predefined_override: Box::new(|_|panic!("Dummy function on test struct called")),
            fn_table_override: Box::new(|_|panic!("Dummy function on test struct called")),
            fn_physical_table_override: Box::new(|_|panic!("Dummy function on test struct called")),
        }
    }
}

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
}
