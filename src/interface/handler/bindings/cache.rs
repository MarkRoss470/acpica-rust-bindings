use core::ffi::CStr;

#[cfg(debug_assertions)]
use alloc::string::{String, ToString};

use alloc::{boxed::Box, vec::Vec};
use bitvec::vec::BitVec;
use log::trace;

use crate::status::{AcpiError, AcpiErrorAsStatusExt, AcpiStatus};

#[derive(Debug)]
struct AcpiCache {
    #[cfg(debug_assertions)]
    cache_name: String,

    size: usize,
    allocated: BitVec<u8>,
    memory: Vec<u8>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AcpiCachePtr(*mut AcpiCache);

impl AcpiCachePtr {
    unsafe fn as_ref(&mut self) -> &mut AcpiCache {
        unsafe { &mut *self.0 }
    }
}

#[export_name = "AcpiOsCreateCache"]
extern "C" fn acpi_os_create_cache(
    cache_name: *mut i8,
    size: u16,
    max_depth: u16,
    return_cache: *mut AcpiCachePtr,
) -> AcpiStatus {
    let cache_name = unsafe { CStr::from_ptr(cache_name) };
    let size = size as usize;
    let max_depth = max_depth as usize;

    trace!(target: "acpi_os_create_cache", "Creating cache called {cache_name:?} with object size {size:#x}, max depth {max_depth:#x}");

    let bitvec_byte_length = max_depth / 8;
    let bitvec = {
        let mut vec = Vec::<u8>::new();
        let Ok(()) = vec.try_reserve_exact(bitvec_byte_length) else {
            return AcpiError::NoMemory.to_acpi_status();
        };

        vec.resize(bitvec_byte_length, 0);

        let bitvec = BitVec::try_from_vec(vec).unwrap();
        debug_assert!(!bitvec.any());

        bitvec
    };

    let memory = {
        let memory_byte_length = size * max_depth;
        
        let mut vec = Vec::<u8>::new();
        let Ok(()) = vec.try_reserve_exact(memory_byte_length) else {
            return AcpiError::NoMemory.to_acpi_status();
        };

        vec.resize(memory_byte_length, 0);

        vec
    };

    trace!(target: "acpi_os_create_cache", "Created cache memory at {:p}", memory.as_ptr());

    let cache = Box::new(AcpiCache {
        #[cfg(debug_assertions)]
        cache_name: cache_name.to_string_lossy().to_string(),
        size,
        allocated: bitvec,
        memory,
    });

    unsafe { *return_cache = AcpiCachePtr(Box::leak(cache)) }

    AcpiStatus::OK
}

#[export_name = "AcpiOsDeleteCache"]
extern "C" fn acpi_os_delete_cache(cache: AcpiCachePtr) -> AcpiStatus {
    let b = unsafe { Box::from_raw(cache.0) };

    #[cfg(debug_assertions)]
    trace!(target: "acpi_os_delete_cache", "Deleting cache {:?}", b.cache_name);

    drop(b);

    AcpiStatus::OK
}

#[export_name = "AcpiOsPurgeCache"]
extern "C" fn acpi_os_purge_cache(mut cache: AcpiCachePtr) -> AcpiStatus {
    let cache = unsafe { cache.as_ref() };

    #[cfg(debug_assertions)]
    trace!(target: "acpi_os_purge_cache", "Purging cache {:?}", cache.cache_name);

    cache.allocated.fill(false);

    AcpiStatus::OK
}

#[export_name = "AcpiOsAcquireObject"]
extern "C" fn acpi_os_acquire_object(mut cache: AcpiCachePtr) -> *mut ::core::ffi::c_void {
    let cache = unsafe { cache.as_ref() };

    #[cfg(debug_assertions)]
    trace!(target: "acpi_os_acquire_object", "Allocating in cache {:?}", cache.cache_name);

    let Some(unallocated_index) = cache.allocated.first_zero() else {
        trace!(target: "acpi_os_acquire_object", "Cache allocation failed");

        return core::ptr::null_mut();
    };

    cache.allocated.set(unallocated_index, true);

    let ptr = cache
        .memory
        .chunks_exact_mut(cache.size)
        .nth(unallocated_index)
        .unwrap()
        .as_mut_ptr()
        .cast();

    trace!(target: "acpi_os_acquire_object", "Allocated {ptr:p}");

    ptr
}

#[export_name = "AcpiOsReleaseObject"]
extern "C" fn acpi_os_release_object(
    mut cache: AcpiCachePtr,
    object: *mut ::core::ffi::c_void,
) -> AcpiStatus {
    let cache = unsafe { cache.as_ref() };

    #[cfg(debug_assertions)]
    trace!(target: "acpi_os_acquire_object", "Removing object at {object:p} from cache {:?}", cache.cache_name);

    let index = (object as usize - cache.memory.as_ptr() as usize) / cache.size;
    cache.allocated.set(index, false);

    AcpiStatus::OK
}
