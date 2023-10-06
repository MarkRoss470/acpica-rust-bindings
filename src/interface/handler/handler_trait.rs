#[cfg(not(all(feature = "builtin_cache", feature = "builtin_lock",)))]
use crate::types::{AcpiAllocationError, AcpiCpuFlags};

#[cfg(not(all(
    feature = "builtin_cache",
    feature = "builtin_lock",
    feature = "builtin_semaphore",
    feature = "builtin_alloc",
)))]
use core::ffi::c_void;

use alloc::string::String;

use crate::{
    interface::status::AcpiError,
    types::{
        AcpiInterruptCallback, AcpiMappingError, AcpiPhysicalAddress, AcpiPredefinedNames,
        AcpiTableHeader, AcpiThreadCallback,
    },
};

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
/// [`register_interface`]: super::register_interface
/// [`allocate`]: AcpiHandler::allocate
pub unsafe trait AcpiHandler {
    /// Method called when ACPICA initialises. The default implementation of this method is a no-op.
    ///
    /// # Safety
    /// * This method is only called from `AcpiOsInitialize`
    unsafe fn initialize(&mut self) -> Result<(), AcpiError> {
        Ok(())
    }

    /// Method called when ACPICA shuts down. The default implementation of this method is a no-op
    ///
    /// # Safety
    /// * This method is only called from `AcpiOsTerminate`
    unsafe fn terminate(&mut self) -> Result<(), AcpiError> {
        Ok(())
    }

    /// Gets a physical pointer to the RSDP.
    ///
    /// # Implementation Safety
    /// * The returned pointer must point to the system's RSDP.
    fn get_root_pointer(&mut self) -> AcpiPhysicalAddress;

    /// Allows the OS to specify an override for a predefined object in the ACPI namespace.
    /// The returned string will be converted to a [`CString`], so the FFI handler for this
    /// method will panic if it contains null bytes.
    ///
    /// # Safety
    /// * This function is only called from `AcpiOsPredefinedOverride`
    ///
    /// [`CString`]: alloc::ffi::CString
    #[allow(unused_variables)]
    unsafe fn predefined_override(
        &mut self,
        predefined_object: &AcpiPredefinedNames,
    ) -> Result<Option<String>, AcpiError> {
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
    ) -> Result<Option<AcpiTableHeader>, AcpiError> {
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

    /// Map `length` bytes of physical memory starting at `physical_address`, and return the virtual address where they have been mapped.
    ///
    /// # Safety
    /// * This function is only called from `AcpiOsMapMemory`
    unsafe fn map_memory(
        &mut self,
        physical_address: AcpiPhysicalAddress,
        length: usize,
    ) -> Result<*mut u8, AcpiMappingError>;

    /// Unmap `length` pages bytes of memory which were previously allocated with [`map_memory`]
    ///
    /// # Safety
    /// * This function is only called from `AcpiOsUnmapMemory`
    /// * `address` is a pointer which was previously returned from [`map_memory`]
    ///
    /// [`map_memory`]: AcpiHandler::map_memory
    unsafe fn unmap_memory(&mut self, address: *mut u8, length: usize);

    /// Translate a logical address to the physical address it's mapped to.
    ///
    /// # Return value
    /// * `Ok(Some(address))`: The translation was successful
    /// * `Ok(None)`: The translation was successful but the virtual address is not mapped
    /// * `Err(e)`: There was an error carrying out the translation
    fn get_physical_address(
        &mut self,
        logical_address: *mut u8,
    ) -> Result<Option<AcpiPhysicalAddress>, AcpiError>;

    /// Register the given `callback` to run in the interrupt handler for the given `interrupt_number`
    ///
    /// # Safety
    /// * This method is only called from `AcpiOsInstallInterruptHandler`.
    unsafe fn install_interrupt_handler(
        &mut self,
        interrupt_number: u32,
        callback: AcpiInterruptCallback,
    ) -> Result<(), AcpiError>;

    /// Remove an interrupt handler which was previously registered with [`install_interrupt_handler`].
    ///
    /// # Safety
    /// * This method is only called from `AcpiOsRemoveInterruptHandler`.
    ///
    /// [`install_interrupt_handler`]: AcpiHandler::install_interrupt_handler
    unsafe fn remove_interrupt_handler(
        &mut self,
        interrupt_number: u32,
        callback: AcpiInterruptCallback,
    ) -> Result<(), AcpiError>;

    /// Gets the thread ID of the kernel thread this method is called from.
    ///
    /// # Implementation safety
    /// * The returned thread ID must be and must be unique to the executing thread.
    /// * The thread ID may not be 0 and may not be equal to [`u64::MAX`]
    fn get_thread_id(&mut self) -> u64;

    /// Run the callback in a new kernel thread.
    ///
    /// # Safety
    /// * This method is only called from `AcpiOsExecute`
    ///
    /// # Return value
    /// * `Ok(())`: The thread is queued and ready to execute
    /// * `Err(e)`: There was an error creating the thread
    unsafe fn execute(
        &mut self,
        // callback_type: AcpiExecuteType,
        callback: AcpiThreadCallback,
    ) -> Result<(), AcpiError>;

    /// Print a message to the kernel's output.
    ///
    /// Multiple calls to `printf` may be used to print a single line of output, and ACPICA will write a newline character at the end of each line.
    /// For this reason, the OS should not add its own newline characters or this could break formatting.
    /// If your kernel has a macro which behaves like the standard `print!` macro, the implementation of this method can be as simple as
    ///
    /// ```ignore
    /// fn printf(&mut self, message: core::fmt::Arguments) {
    ///     print!("{message}");
    /// }
    /// ```
    fn printf(&mut self, message: core::fmt::Arguments);

    // fn read_memory(
    //     &mut self,
    //     address: AcpiPhysicalAddress,
    //     value: &mut [u64],
    //     width: u32,
    // ) -> Result<(), AcpiError>;

    // fn write_memory(
    //     &mut self,
    //     address: AcpiPhysicalAddress,
    //     value: u64,
    //     width: u32,
    // ) -> Result<(), AcpiError>;

    // fn readable(&mut self, Pointer: *mut ::core::ffi::c_void, Length: usize) -> bool;

    // fn writable(&mut self, Pointer: *mut ::core::ffi::c_void, Length: usize) -> bool;

    // TODO: Verify the info in the docs for these cache methods

    /// Creates a cache for ACPICA to store objects in to avoid lots of small heap allocations.
    ///
    /// This method is only present in the trait if the `builtin_cache` feature is not set.
    /// Otherwise, a default implementation is used which allocates the objects inside a [`Vec`],
    /// using a [`BitVec`] to keep track of which objects within the cache have been allocated.
    ///
    /// The cache stores up to `max_depth` objects of size `object_size`.
    /// The OS is responsible for allocating and de-allocating objects within the cache.
    ///
    /// The OS returns a type-erased pointer which can safely be passed via FFI,
    /// but the pointer may point to any type.
    ///
    /// # Safety
    /// * This method is only called from `AcpiCreateCache`.
    ///
    /// [`Vec`]: alloc::vec::Vec
    /// [`BitVec`]: bitvec::vec::BitVec
    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn create_cache(
        &mut self,
        cache_name: &str,
        object_size: u16,
        max_depth: u16,
    ) -> Result<*mut c_void, AcpiError>;

    /// Deletes a cache which was previously created by [`create_cache`].
    ///
    /// This method is only present in the trait if the `builtin_cache` feature is not set.
    ///
    /// The OS is responsible for deallocating the backing memory of the cache.
    ///
    /// # Safety
    /// * This method is only called from `AcpiDeleteCache`.
    /// * `cache` is a pointer which was previously returned from [`create_cache`].
    /// * After this method is called, other cache methods will not be called for this cache.
    ///
    /// [`create_cache`]: AcpiHandler::create_cache
    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn delete_cache(&mut self, cache: *mut c_void) -> Result<(), AcpiAllocationError>;

    /// Removes all items from a cache.
    ///
    /// This method is only present in the trait if the `builtin_cache` feature is not set.
    ///
    /// This method should mark all slots in the cache as empty, but not deallocate the backing memory.
    ///
    /// # Safety
    /// * This method is only called from `AcpiPurgeCache`
    /// * `cache` is a pointer which was previously returned from [`create_cache`]
    ///
    /// [`create_cache`]: AcpiHandler::create_cache
    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn purge_cache(&mut self, cache: *mut c_void);

    /// Allocates an object inside a cache.
    ///
    /// This method is only present in the trait if the `builtin_cache` feature is not set.
    ///
    /// This method should return a pointer to a free slot in the cache, or `None` if all slots are full.
    ///
    /// # Safety
    /// * This method is only called from `AcpiPurgeCache`.
    /// * `cache` is a pointer which was previously returned from [`create_cache`].
    ///
    /// # Implementation safety
    /// * The returned pointer must be free for writes for the object size passed to [`create_cache`]
    ///     - i.e. it must not be being used by rust code, and it must not have been returned from this method before,
    ///     unless it has been explicitly freed using [`release_object`] or [`purge_cache`].
    ///
    /// [`create_cache`]: AcpiHandler::create_cache
    /// [`release_object`]: AcpiHandler::release_object
    /// [`purge_cache`]: AcpiHandler::purge_cache
    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn acquire_object(&mut self, cache: *mut c_void) -> Option<*mut u8>;

    /// Marks an object as free in a cache.
    ///
    /// This method is only present in the trait if the `builtin_cache` feature is not set.
    ///
    /// This method should mark the given object within the cache as free - i.e. allow it to be allocated again by [`acquire_object`].
    ///
    /// # Safety
    /// * This method is only called from `AcpiReleaseObject`.
    /// * `cache` is a pointer which was previously returned from [`create_cache`].
    /// * `object` is a pointer which was previously returned from [`acquire_object`].
    ///
    /// [`acquire_object`]: AcpiHandler::acquire_object
    /// [`create_cache`]: AcpiHandler::create_cache
    #[cfg(not(feature = "builtin_cache"))]
    unsafe fn release_object(&mut self, cache: *mut c_void, object: *mut u8);

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn create_lock(&mut self) -> Result<*mut c_void, AcpiError>;

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn delete_lock(&mut self, lock: *mut c_void);

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn acquire_lock(&mut self, handle: *mut c_void) -> AcpiCpuFlags;

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_lock"))]
    unsafe fn release_lock(&mut self, handle: *mut c_void, flags: AcpiCpuFlags);

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn create_semaphore(
        &mut self,
        max_units: u32,
        initial_units: u32,
    ) -> Result<*mut c_void, AcpiError>;

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn delete_semaphore(&mut self, handle: *mut c_void) -> Result<(), AcpiError>;

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn wait_semaphore(
        &mut self,
        handle: *mut c_void,
        units: u32,
        timeout: u16,
    ) -> Result<(), AcpiError>;

    #[allow(missing_docs)] // TODO: docs
    #[cfg(not(feature = "builtin_semaphore"))]
    unsafe fn signal_semaphore(&mut self, handle: *mut c_void, units: u32)
        -> Result<(), AcpiError>;

    /// Allocate `size` bytes of memory.
    ///
    /// This method is only present in the trait if the `builtin_alloc` feature is not set.
    /// Otherwise, a default implementation is used which forwards allocations to the system allocator.
    ///
    /// The implementation of this method is not as straightforward as it may seem, as rust's heap allocator
    /// API requires the length to be passed to [`dealloc`] as part of the
    /// [`Layout`], but the ACPICA [`free`] function doesn't pass it.
    /// The default implementations of these methods stores the allocated size in the first few bytes of the
    /// allocation and returning the rest to ACPICA.
    ///
    /// # Safety
    /// * This method is only called from `AcpiOsAllocate`
    /// * The returned pointer must only be used to access `size` bytes.
    ///
    /// [`dealloc`]: alloc::alloc::dealloc
    /// [`Layout`]: alloc::alloc::Layout
    /// [`free`]: AcpiHandler::free
    #[cfg(not(feature = "builtin_alloc"))]
    unsafe fn allocate(&mut self, size: usize) -> Result<*mut u8, AcpiAllocationError>;

    // TODO: native allocate zeroed (see bindings.rs)
    // fn allocate_zeroed(&mut self, Size: usize) -> *mut ::core::ffi::c_void;

    /// Free the allocation at `memory`.
    ///
    /// This method is only present in the trait if the `builtin_alloc` feature is not set.
    /// Otherwise, a default implementation is used which forwards allocations to the system allocator.
    ///
    /// See the docs for [`allocate`] for potential problems implementing this method.
    ///
    /// # Safety
    /// * `memory` must be a pointer which was allocated using [`allocate`]
    ///
    /// [`allocate`]: AcpiHandler::allocate
    #[cfg(not(feature = "builtin_alloc"))]
    unsafe fn free(&mut self, memory: *mut u8);
}
