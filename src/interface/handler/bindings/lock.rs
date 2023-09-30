use core::sync::atomic::{AtomicBool, Ordering};

use alloc::boxed::Box;

use crate::{
    bindings::types::FfiAcpiCpuFlags,
    status::{AcpiError, AcpiErrorAsStatusExt, AcpiStatus},
};

#[derive(Debug)]
#[repr(transparent)]
pub struct AcpiSpinlock(*const AtomicBool);

impl AcpiSpinlock {
    unsafe fn as_ref(&self) -> &AtomicBool {
        unsafe { &*self.0 }
    }
}

#[export_name = "AcpiOsCreateLock"]
extern "C" fn acpi_os_create_lock(out_handle: *mut AcpiSpinlock) -> AcpiStatus {
    if out_handle.is_null() {
        return AcpiError::BadParameter.to_acpi_status();
    }

    let lock = Box::new(AtomicBool::new(false)); // TODO: make this fallible

    unsafe { *out_handle = AcpiSpinlock(Box::leak(lock)) }

    AcpiStatus::OK
}

#[export_name = "AcpiOsDeleteLock"]
extern "C" fn acpi_os_delete_lock(handle: AcpiSpinlock) {
    // SAFETY: This function is only called if there are no more references to this spinlock
    // Therefore it is safe to cast the pointer to mutable and construct a box from it.
    let b = unsafe { Box::from_raw(handle.0.cast_mut()) };

    drop(b);
}

#[export_name = "AcpiOsAcquireLock"]
extern "C" fn acpi_os_acquire_lock(handle: AcpiSpinlock) -> FfiAcpiCpuFlags {
    let handle = unsafe { handle.as_ref() };

    loop {
        if handle
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_ok()
        {
            return 0;
        }
    }
}

#[export_name = "AcpiOsReleaseLock"]
extern "C" fn acpi_os_release_lock(handle: AcpiSpinlock, _flags: FfiAcpiCpuFlags) {
    let handle = unsafe { handle.as_ref() };

    handle.store(false, Ordering::Release);
}
