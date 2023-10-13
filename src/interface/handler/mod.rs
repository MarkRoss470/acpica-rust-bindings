//! The [`AcpiHandler`] trait, which is the interface with which ACPICA calls OS functions.

mod bindings;
mod handler_trait;

pub use handler_trait::AcpiHandler;

use core::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use alloc::{boxed::Box, ffi::CString, vec::Vec};
use spin::Mutex;

use crate::bindings::{
    consts::ACPI_FULL_INITIALIZATION,
    functions::{
        AcpiEnableSubsystem, AcpiEnterSleepStatePrep, AcpiInitializeObjects,
        AcpiInitializeSubsystem, AcpiInitializeTables, AcpiLoadTables, AcpiEnterSleepState, AcpiGetTimer,
    },
};

use super::status::AcpiError;

static OS_INTERFACE: Mutex<Option<OsInterface>> = Mutex::new(None);

// TODO: Are these still needed when using type state to store the same flags?
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
/// See the docs for [`AcpicaInitialization`] for more info.
///
/// # Panics
/// If called more than once.
pub fn register_interface<T: AcpiHandler + Send + 'static>(
    interface: T,
) -> Result<AcpicaInitialization<false, false>, AcpiError> {
    let mut lock = OS_INTERFACE.lock();

    assert!(!lock.is_some(), "Interface is already initialized");

    *lock = Some(OsInterface {
        handler: Box::new(interface),
        objects_to_drop: Vec::new(),
    });

    SUBSYSTEM_IS_INITIALIZED.store(true, Ordering::Relaxed);

    // AcpiInitializeSubsystem calls functions which need this lock
    drop(lock);

    // SAFETY: Handlers for AcpiOs functions have been set up
    unsafe { AcpiInitializeSubsystem().as_result()? };

    Ok(AcpicaInitialization)
}

/// The state of ACPICA's initialization, represented using type-state.
///
/// ACPICA is not initialized all at once - it has multiple initialization functions for different systems.
/// This struct keeps track of the calling of these functions using the type parameters `T`, for whether ACPICA tables are initialized,
/// and `S` for whether the ACPICA subsystem is enabled. An `AcpiInitialization<false, false>` can be obtained from [`register_interface`],
/// which also calls `AcpiInitializeSubsystem`.
///
/// Subsystem initialization code could look like the following:
/// ```ignore
/// # use acpica_bindings::status::AcpiError;
/// # use acpica_bindings::handler::register_interface;
/// # fn main() -> Result<(), AcpiError> {
///     let interface = todo!(); // In real code this would be an object implementing the AcpiHandler trait
///
///     let initialization = register_interface(interface)?;
///     let initialization = initialization.load_tables()?;
///     let initialization = initialization.enable_subsystem()?;
///     let initialization = initialization.initialize_objects()?;
///
/// #   Ok(())
/// # }
/// ```
#[derive(Debug)]
#[must_use]
pub struct AcpicaInitialization<const T: bool, const S: bool>;

impl AcpicaInitialization<false, false> {
    /// Calls the ACPICA functions `AcpiInitializeTables` and `AcpiLoadTables`.
    ///
    /// This function causes ACPICA to parse all the tables pointed to by the RSDT/XSDT
    pub fn load_tables(self) -> Result<AcpicaInitialization<true, false>, AcpiError> {
        // SAFETY: `AcpiInitializeSubsystem` has been called
        unsafe { AcpiInitializeTables(core::ptr::null_mut(), 16, false).as_result()? };
        // SAFETY: `AcpiInitializeTables` has been called
        unsafe { AcpiLoadTables().as_result()? };

        TABLES_ARE_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(AcpicaInitialization)
    }
}

impl AcpicaInitialization<true, false> {
    /// Calls the ACPICA function `AcpiEnableSubsystem`.
    ///
    /// This function causes ACPICA to enter ACPI mode and start receiving ACPI interrupts.
    pub fn enable_subsystem(self) -> Result<AcpicaInitialization<true, true>, AcpiError> {
        // SAFETY: `AcpiLoadTables` has been called
        unsafe { AcpiEnableSubsystem(ACPI_FULL_INITIALIZATION).as_result()? };

        SUBSYSTEM_IS_ENABLED.store(true, Ordering::Relaxed);

        Ok(AcpicaInitialization)
    }
}

impl AcpicaInitialization<true, true> {
    /// Calls the ACPICA function `AcpiEnableSubsystem`.
    ///
    /// This function causes ACPICA to enter ACPI mode and start receiving ACPI interrupts.
    pub fn initialize_objects(self) -> Result<AcpicaOperation, AcpiError> {
        // SAFETY: `AcpiEnableSubsystem` has been called
        unsafe { AcpiInitializeObjects(ACPI_FULL_INITIALIZATION).as_result()? };

        OBJECTS_ARE_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(AcpicaOperation)
    }
}

/// The type on which ACPICA functions are exposed as methods.
/// This type is obtained from the [`initialize_objects`] method on an [`AcpicaInitialization`]
/// This type doesn't store any state, but the presence of an instance of the state indicates that ACPICA is fully set up.
/// 
/// [`initialize_objects`]: AcpicaInitialization::initialize_objects
#[derive(Debug)]
pub struct AcpicaOperation;

impl AcpicaOperation {
    /// TODO: docs
    pub unsafe fn enter_sleep_state_prep(&mut self, state: u8) -> Result<(), AcpiError> {
        // SAFETY: TODO
        unsafe { AcpiEnterSleepStatePrep(state).as_result() }
    }

    /// TODO: docs
    pub unsafe fn enter_sleep_state(&mut self, state: u8) -> Result<(), AcpiError> {
        // SAFETY: TODO
        unsafe { AcpiEnterSleepState(state).as_result() }
    }

    /// TODO: docs
    pub fn get_timer(&mut self) -> u32 {
        let mut x = 0;

        // SAFETY: TODO
        unsafe { AcpiGetTimer(&mut x).as_result().unwrap() }

        x
    }
}