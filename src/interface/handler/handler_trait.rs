use alloc::{vec::Vec, boxed::Box};

use crate::{interface::status::AcpiError, types::{AcpiPhysicalAddress, AcpiPredefinedNames, AcpiString, AcpiTableHeader, AcpiSpinLock, AcpiSize, AcpiSemaphore, AcpiAllocationError, AcpiCache, AcpiCallback, AcpiExecuteType}};

/// The interface between ACPICA and the host OS. Each method in this trait is mapped to an `AcpiOs...` function,
/// which will be called on the object registered with [`register_interface`]. 
/// 
/// Some methods such as [`allocate`] which can be no-ops or which map nicely to rust concepts have default implementations, 
/// but most of the methods are very OS-specific and must be implemented uniquely for each host OS.
/// 
/// # Safety
/// This trait is unsafe to implement because some functions have restrictions on their 
/// implementation as well as their caller. This is indicated per method under the heading "Implementation Safety".
/// 
/// [`allocate`]: AcpiHandler::allocate
pub unsafe trait AcpiHandler {
    /// Method called when ACPICA initialises. The default implementation of this method is a no-op.
    /// 
    /// # Safety
    /// This method is only called from `AcpiOsInitialize`
    unsafe fn initialize(&mut self) -> Result<(), AcpiError> {
        Ok(())
    }

    /// Method called when ACPICA shuts down. The default implementation of this method is a no-op
    /// 
    /// # Safety
    /// This method is only called from `AcpiOsTerminate`
    unsafe fn terminate(&mut self) -> Result<(), AcpiError> {
        Ok(())
    }

    /// Gets a physical pointer to the RSDP.
    /// 
    /// # Implementation Safety
    /// * The returned pointer must point to the system's RSDP.
    fn get_root_pointer(&mut self) -> AcpiPhysicalAddress;

    /// Allows the OS to specify an override for a predefined object in the ACPI namespace.
    /// 
    /// # Safety
    /// * This function is only called from `AcpiOsPredefinedOverride`
    #[allow(unused_variables)]
    unsafe fn predefined_override(
        &mut self,
        predefined_object: &AcpiPredefinedNames,
    ) -> Result<Option<AcpiString>, AcpiError> {
        Ok(None)
    }

    /// Allows the OS to override an ACPI table using a logical address.
    /// This method is called once on each ACPI table in the order they are listed in the DSDT/XSDT, 
    /// and when tables are loaded by the `Load` AML instruction. To keep the original table, return `Ok(None)`.
    /// 
    /// To override the table using a physical address instead, use [`physical_table_override`]
    /// 
    /// # Safety
    /// * This method is only called from `AcpiOsTableOverride`.
    /// 
    /// [`physical_table_override`]: AcpiHandler::physical_table_override
    #[allow(unused_variables)]
    unsafe fn table_override(
        &mut self,
        table: &AcpiTableHeader,
    ) -> Result<Option<&'static mut AcpiTableHeader>, AcpiError> {
        Ok(None)
    }

    /// Allows the OS to override an ACPI table using a physical address. 
    /// To keep the original table, return `Ok(None)`
    /// 
    /// # Safety
    /// * This method is only called from `AcpiOsPhysicalTableOverride`.
    /// 
    /// # Implementation Safety
    /// * The returned physical address must point to a valid new ACPI table with the returned length
    /// * The memory indicated by the returned pointer and length is now managed by ACPICA and must
    ///     not be written to while ACPICA is active.
    #[allow(unused_variables)]
    unsafe fn physical_table_override(
        &mut self,
        table: &AcpiTableHeader,
    ) -> Result<Option<(AcpiPhysicalAddress, u32)>, AcpiError> {
        Ok(None)
    }

    fn create_lock(&mut self) -> Result<AcpiSpinLock, AcpiError>;

    fn delete_lock(&mut self, hock: AcpiSpinLock);

    fn acquire_lock(&mut self, handle: AcpiSpinLock) -> AcpiSize;

    fn release_lock(&mut self, handle: AcpiSpinLock, flags: AcpiSize);

    fn create_semaphore(
        &mut self,
        max_units: u32,
        initial_units: u32,
    ) -> Result<AcpiSemaphore, AcpiError>;

    fn delete_semaphore(&mut self) -> Result<AcpiSemaphore, AcpiError>;

    fn wait_semaphore(
        &mut self,
        handle: AcpiSemaphore,
        units: u32,
        timeout: u16,
    ) -> Result<(), AcpiError>;

    fn signal_semaphore(&mut self, handle: AcpiSemaphore, units: u32) -> Result<(), AcpiError>;

    /// Allocate `size` bytes of memory.
    /// The default implementation of this method uses the global heap allocator.
    ///
    /// The implementation of this method is not as straightforward as it may seem, as rust's heap allocator
    /// API requires the length to be passed to [`dealloc`] as part of the
    /// [`Layout`], but the ACPICA [`free`] function doesn't pass it.
    /// The default implementations of these methods stores the allocated size in the first few bytes of the
    /// allocation and returning the rest to ACPICA.
    ///
    /// # Safety
    /// * The returned pointer must only be used to access `size` bytes.
    /// 
    /// # Implementation Safety
    /// * If this method is overridden, [`free`] must be as well with a matching implementation.
    /// * The pointer returned must point to at least `size` bytes of memory which is valid for reads and writes.
    /// 
    /// [`dealloc`]: alloc::alloc::dealloc
    /// [`Layout`]: alloc::alloc::Layout
    /// [`free`]: AcpiHandler::free
    unsafe fn allocate(&mut self, size: AcpiSize) -> Result<*mut u8, AcpiAllocationError> {
        let mut v = Vec::<u8>::new();

        let total_allocation_size = size.as_usize() + core::mem::size_of::<usize>();
        let Ok(_) = v.try_reserve_exact(total_allocation_size) else {
            return Err(AcpiAllocationError);
        };

        assert_eq!(v.capacity(), total_allocation_size);

        let ptr = v.as_mut_ptr();
        core::mem::forget(v);

        // SAFETY:
        // There are no references to the Vec any more, so writing to its memory is sound.
        unsafe { core::ptr::write_unaligned(ptr as *mut usize, size.as_usize()) }

        // SAFETY: This is adding <=8 bytes so it can't exceed the size bounds
        Ok(unsafe { ptr.byte_add(core::mem::size_of::<usize>()) })
    }

    // TODO: native allocate zeroed (see bindings.rs)
    // fn allocate_zeroed(&mut self, Size: AcpiSize) -> *mut ::core::ffi::c_void;

    /// Free the allocation at `memory`.
    /// See the docs for [`allocate`] for potential problems implementing this method.
    /// 
    /// # Safety
    /// * `memory` must be a pointer which was allocated using [`allocate`]
    /// 
    /// # Implementation Safety
    /// * If this function is overridden, [`allocate`] must be as well with a matching implementation.
    /// 
    /// [`allocate`]: AcpiHandler::allocate
    unsafe fn free(&mut self, memory: *mut u8) {
        // SAFETY: The pointer passed to ACPICA was one usize more than the actual allocated Vec,
        // so this pointer is part of the same allocation.
        let real_start = unsafe { memory.byte_sub(core::mem::size_of::<usize>()) };
        // SAFETY: A usize was written here in `allocate`, so it can be read here.
        let size = unsafe { core::ptr::read_unaligned(real_start as *const usize) };
        // SAFETY: This pointer was allocated by Vec with this size and capacity in `allocate`.
        let v = unsafe { Vec::from_raw_parts(real_start, size, size) };

        // Explicitly drop the Vec
        drop(v);
    }

    fn map_memory(&mut self, physical_address: AcpiPhysicalAddress, length: AcpiSize) -> *mut u8;

    fn unmap_memory(&mut self, address: *mut u8, size: AcpiSize);

    fn get_physical_address(
        &mut self,
        logical_address: *mut u8,
    ) -> Result<AcpiPhysicalAddress, AcpiError>;

    fn create_cache(
        &mut self,
        cache_name: AcpiString,
        object_size: u16,
        max_depth: u16,
    ) -> Result<AcpiCache, AcpiError>;

    fn delete_cache(&mut self, cache: &mut AcpiCache) -> Result<(), AcpiError>;

    fn purge_cache(&mut self, cache: &mut AcpiCache) -> Result<(), AcpiError>;

    fn acquire_object(&mut self, cache: &mut AcpiCache) -> *mut u8;

    fn release_object(&mut self, cache: &mut AcpiCache, object: *mut u8) -> Result<(), AcpiError>;

    fn install_interrupt_handler(
        &mut self,
        interrupt_number: u32,
        service_routine: AcpiCallback,
        context: *mut u8,
    ) -> Result<(), AcpiError>;

    fn remove_interrupt_handler(
        &mut self,
        interrupt_number: u32,
        service_routine: AcpiCallback,
    ) -> Result<(), AcpiError>;

    fn get_thread_id(&mut self) -> u64;

    fn execute(
        &mut self,
        callback_type: AcpiExecuteType,
        callback: Box<dyn Fn()>,
    ) -> Result<(), AcpiError>;

    // fn wait_events_complete(&mut self, );

    // fn sleep(&mut self, Milliseconds: u64);

    // fn stall(&mut self, Microseconds: u32);

    // fn read_port(&mut self,
    //     Address: ACPI_IO_ADDRESS,
    //     Value: *mut u32,
    //     Width: u32,
    // ) -> Result<(), AcpiError>;

    // fn write_port(&mut self, Address: ACPI_IO_ADDRESS, Value: u32, Width: u32) -> Result<(), AcpiError>;

    // fn read_memory(&mut self,
    //     Address: ACPI_PHYSICAL_ADDRESS,
    //     Value: *mut u64,
    //     Width: u32,
    // ) -> Result<(), AcpiError>;

    // fn write_memory(&mut self,
    //     Address: ACPI_PHYSICAL_ADDRESS,
    //     Value: u64,
    //     Width: u32,
    // ) -> Result<(), AcpiError>;

    // fn read_pci_configuration(&mut self,
    //     PciId: *mut ACPI_PCI_ID,
    //     Reg: u32,
    //     Value: *mut u64,
    //     Width: u32,
    // ) -> Result<(), AcpiError>;

    // fn write_pci_configuration(&mut self,
    //     PciId: *mut ACPI_PCI_ID,
    //     Reg: u32,
    //     Value: u64,
    //     Width: u32,
    // ) -> Result<(), AcpiError>;

    // fn readable(&mut self, Pointer: *mut ::core::ffi::c_void, Length: AcpiSize) -> bool;

    // fn writable(&mut self, Pointer: *mut ::core::ffi::c_void, Length: AcpiSize) -> bool;

    // fn get_timer(&mut self, ) -> u64;

    // fn signal(&mut self, Function: u32, Info: *mut ::core::ffi::c_void) -> Result<(), AcpiError>;

    // fn enter_sleep(&mut self, SleepState: u8, RegaValue: u32, RegbValue: u32)
    //     -> Result<(), AcpiError>;

    // fn printf(&mut self, Format: *const i8, ...);

    // fn redirect_output(&mut self, Destination: *mut ::core::ffi::c_void);

    // fn get_line(&mut self,
    //     Buffer: *mut i8,
    //     BufferLength: u32,
    //     BytesRead: *mut u32,
    // ) -> Result<(), AcpiError>;

    // fn initialize_debugger(&mut self, ) -> Result<(), AcpiError>;

    // fn terminate_debugger(&mut self, );

    // fn wait_command_ready(&mut self, ) -> Result<(), AcpiError>;

    // fn notify_command_complete(&mut self, ) -> Result<(), AcpiError>;

    // fn trace_point(&mut self,
    //     Type: ACPI_TRACE_EVENT_TYPE,
    //     Begin: bool,
    //     Aml: *mut u8,
    //     Pathname: *mut i8,
    // );

    // fn get_table_by_name(&mut self,
    //     Signature: *mut i8,
    //     Instance: u32,
    //     Table: *mut *mut AcpiTableHeader,
    //     Address: *mut ACPI_PHYSICAL_ADDRESS,
    // ) -> Result<(), AcpiError>;

    // fn get_table_by_index(&mut self,
    //     Index: u32,
    //     Table: *mut *mut AcpiTableHeader,
    //     Instance: *mut u32,
    //     Address: *mut ACPI_PHYSICAL_ADDRESS,
    // ) -> Result<(), AcpiError>;

    // fn get_table_by_address(&mut self,
    //     Address: ACPI_PHYSICAL_ADDRESS,
    //     Table: *mut *mut AcpiTableHeader,
    // ) -> Result<(), AcpiError>;

    // fn open_directory(&mut self,
    //     Pathname: *mut i8,
    //     WildcardSpec: *mut i8,
    //     RequestedFileType: i8,
    // ) -> *mut ::core::ffi::c_void;

    // fn get_next_filename(&mut self, DirHandle: *mut ::core::ffi::c_void) -> *mut i8;

    // fn close_directory(&mut self, DirHandle: *mut ::core::ffi::c_void);

    // fn vprintf(&mut self, _format: *const u8, _args: ...);
}
