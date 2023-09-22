mod bindings;
mod handler_trait;

pub use handler_trait::AcpiHandler;

use core::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use alloc::{boxed::Box, string::String, vec::Vec, ffi::CString};
use spin::Mutex;

use crate::bindings::{
    consts::ACPI_FULL_INITIALIZATION,
    functions::{
        AcpiEnableSubsystem, AcpiInitializeObjects, AcpiInitializeSubsystem, AcpiInitializeTables,
        AcpiLoadTables,
    },
};

use super::status::AcpiError;

static OS_INTERFACE: Mutex<Option<OsInterface>> = Mutex::new(None);

pub(crate) static SUBSYSTEM_IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static TABLES_ARE_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static SUBSYSTEM_IS_ENABLED: AtomicBool = AtomicBool::new(false);
pub(crate) static OBJECTS_ARE_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
enum DropOnTerminate {
    CString(CString),
}

struct OsInterface {
    handler: Box<dyn AcpiHandler + Send>,
    objects_to_drop: Vec<DropOnTerminate>,
}

impl Deref for OsInterface {
    type Target = Box<dyn AcpiHandler + Send>;

    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

impl DerefMut for OsInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handler
    }
}

/// Registers `interface` as the handler for ACPICA functions, and starts the initialization of ACPICA.
///
/// # Panics
/// If called more than once.
pub fn register_interface<T: AcpiHandler + Send + 'static>(
    interface: T,
) -> Result<AcpicaInitialization<false, false>, AcpiError> {
    let mut lock = OS_INTERFACE.lock();

    if lock.is_some() {
        panic!("Interface is already initialized");
    }

    *lock = Some(OsInterface {
        handler: Box::new(interface),
        objects_to_drop: Vec::new(),
    });
    SUBSYSTEM_IS_INITIALIZED.store(true, Ordering::Relaxed);

    // AcpiInitializeSubsystem calls functions which need this lock
    drop(lock);

    unsafe { AcpiInitializeSubsystem().as_result()? };

    // unsafe { AcpiInitializeTables().as_result()? };
    // unsafe { AcpiInitializeSubsystem().as_result()? };

    Ok(AcpicaInitialization)
}

#[derive(Debug)]
#[must_use]
pub struct AcpicaInitialization<const TABLES: bool, const SUBSYSTEM_ENABLE: bool>;

impl AcpicaInitialization<false, false> {
    pub fn load_tables(self) -> Result<AcpicaInitialization<true, false>, AcpiError> {
        unsafe { AcpiInitializeTables(core::ptr::null_mut(), 16, false).as_result()? };
        unsafe { AcpiLoadTables().as_result()? };

        TABLES_ARE_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(AcpicaInitialization)
    }
}

impl AcpicaInitialization<true, false> {
    pub fn enable_subsystem(self) -> Result<AcpicaInitialization<true, true>, AcpiError> {
        unsafe { AcpiEnableSubsystem(ACPI_FULL_INITIALIZATION).as_result()? };

        SUBSYSTEM_IS_ENABLED.store(true, Ordering::Relaxed);

        Ok(AcpicaInitialization)
    }
}

impl AcpicaInitialization<true, true> {
    pub fn initialize_objects(self) -> Result<(), AcpiError> {
        unsafe { AcpiInitializeObjects(ACPI_FULL_INITIALIZATION).as_result()? };

        OBJECTS_ARE_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(())
    }
}
