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

use crate::{
    bindings::{
        consts::ACPI_FULL_INITIALIZATION,
        functions::{
            AcpiEnableSubsystem, AcpiEnterSleepState, AcpiEnterSleepStatePrep, AcpiGetTable,
            AcpiGetTableByIndex, AcpiGetTimer, AcpiInitializeObjects, AcpiInitializeSubsystem,
            AcpiInitializeTables, AcpiLoadTables,
        },
        types::tables::FfiAcpiTableHeader,
    },
    types::tables::{madt::Madt, uefi::Uefi, AcpiTableHeader},
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
) -> Result<AcpicaOperation<false, false>, AcpiError> {
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

    Ok(AcpicaOperation)
}

/// The interface to ACPICA functions. The state of ACPICA's initialization is tracked using const generics on this type.
///
/// ACPICA is not initialized all at once - it has multiple initialization functions for different systems.
/// This struct keeps track of the calling of these functions using the type parameters `T`, for whether ACPICA tables are initialized,
/// and `S` for whether the ACPICA subsystem is enabled. An `AcpiOperation<false, false>` can be obtained from [`register_interface`],
/// which also calls `AcpiInitializeSubsystem`.
///
/// Subsystem initialization code could look like the following:
///
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
pub struct AcpicaOperation<const T: bool = true, const S: bool = true>;

impl AcpicaOperation<false, false> {
    /// Calls the ACPICA functions `AcpiInitializeTables` and `AcpiLoadTables`.
    ///
    /// This function causes ACPICA to parse all the tables pointed to by the RSDT/XSDT
    pub fn load_tables(self) -> Result<AcpicaOperation<true, false>, AcpiError> {
        // SAFETY: `AcpiInitializeSubsystem` has been called
        unsafe { AcpiInitializeTables(core::ptr::null_mut(), 16, false).as_result()? };
        // SAFETY: `AcpiInitializeTables` has been called
        unsafe { AcpiLoadTables().as_result()? };

        TABLES_ARE_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(AcpicaOperation)
    }
}

impl AcpicaOperation<true, false> {
    /// Calls the ACPICA function `AcpiEnableSubsystem`.
    ///
    /// This function causes ACPICA to enter ACPI mode and start receiving ACPI interrupts.
    pub fn enable_subsystem(self) -> Result<AcpicaOperation<true, true>, AcpiError> {
        // SAFETY: `AcpiLoadTables` has been called
        unsafe { AcpiEnableSubsystem(ACPI_FULL_INITIALIZATION).as_result()? };

        SUBSYSTEM_IS_ENABLED.store(true, Ordering::Relaxed);

        Ok(AcpicaOperation)
    }
}

impl<const S: bool> AcpicaOperation<true, S> {
    fn get_tables_of_type(&self, signature: [u8; 4]) -> impl Iterator<Item = AcpiTableHeader> {
        let mut i = 1;

        core::iter::from_fn(move || {
            let mut table = core::ptr::null_mut();
            // Move the signature into an array for borrow checking reasons
            // And so that if ACPICA mutates this string it's not UB
            let mut signature = signature;

            // SAFETY: The signature is valid
            let r = unsafe { AcpiGetTable(signature.as_mut_ptr().cast(), i, &mut table) };

            i += 1;

            match r.as_result() {
                Ok(()) => {
                    assert!(!table.is_null());
                    // SAFETY: The returned pointer is valid
                    unsafe { Some(AcpiTableHeader::from_ffi(&*table.cast_const())) }
                }
                Err(AcpiError::BadParameter) => panic!("ACPICA reported bad parameter"),
                Err(_) => None,
            }
        })
    }

    /// Gets a table by its signature, if it is present on the system.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn table(&self, signature: [u8; 4]) -> Option<AcpiTableHeader> {
        let mut table = core::ptr::null_mut();
        // Move the signature into an array for borrow checking reasons
        // And so that if ACPICA mutates this string it's not UB
        let mut signature = signature;

        // SAFETY: The signature is valid
        let r = unsafe { AcpiGetTable(signature.as_mut_ptr().cast(), 1, &mut table) };

        match r.as_result() {
            Ok(()) => {
                assert!(!table.is_null());
                // SAFETY: The returned pointer is valid
                unsafe { Some(AcpiTableHeader::from_ffi(&*table)) }
            }
            Err(_) => None,
        }
    }

    /// Returns an iterator over all the tables detected by ACPICA
    #[allow(clippy::missing_panics_doc)]
    pub fn tables(&self) -> impl Iterator<Item = AcpiTableHeader> {
        let mut i = 1;

        core::iter::from_fn(move || {
            let mut ptr = core::ptr::null_mut();

            // SAFETY:
            let r = unsafe { AcpiGetTableByIndex(i, &mut ptr) };

            i += 1;

            match r.as_result() {
                Ok(()) => {
                    assert!(!ptr.is_null());
                    // SAFETY: The returned pointer is valid
                    unsafe { Some(AcpiTableHeader::from_ffi(&*ptr)) }
                }
                Err(_) => None,
            }
        })
    }

    /// Gets an iterator over all of the loaded SSDT tables.
    pub fn ssdt_tables(&self) -> impl Iterator<Item = AcpiTableHeader> {
        self.get_tables_of_type(*b"SSDT")
    }

    /// Gets an iterator over all of the loaded UEFI tables.
    pub fn uefi_tables(&self) -> impl Iterator<Item = Uefi> {
        self.get_tables_of_type(*b"UEFI").map(|header| {
            let ptr = header.as_ffi() as *const FfiAcpiTableHeader;
            let ptr = ptr.cast();
            // SAFETY: The signature is "UEFI" so the table can be cast to an FfiAcpiTableUefi
            unsafe { Uefi::from_ffi(&*ptr) }
        })
    }

    /// Gets the DSDT
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn dsdt(&self) -> AcpiTableHeader {
        self.table(*b"DSDT")
            .expect("System should have contained DSDT")
    }

    /// Gets the MADT
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn madt(&self) -> Madt {
        let ptr = self
            .table(*b"APIC")
            .expect("System should have contained MADT")
            .as_ffi() as *const FfiAcpiTableHeader;
        let ptr = ptr.cast();
        // SAFETY: The signature is "APIC" so the table is an MADT
        unsafe { Madt::from_ffi(&*ptr) }
    }
}

impl AcpicaOperation<true, true> {
    /// Calls the ACPICA function `AcpiEnableSubsystem`.
    ///
    /// This function causes ACPICA to enter ACPI mode and start receiving ACPI interrupts.
    pub fn initialize_objects(self) -> Result<AcpicaOperation<true, true>, AcpiError> {
        // SAFETY: `AcpiEnableSubsystem` has been called
        unsafe { AcpiInitializeObjects(ACPI_FULL_INITIALIZATION).as_result()? };

        OBJECTS_ARE_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(AcpicaOperation)
    }
}

impl AcpicaOperation<true, true> {
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
