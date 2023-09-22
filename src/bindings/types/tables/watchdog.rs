use crate::{bindings::types::FfiAcpiTableHeader, interface::AcpiGenericAddress};

///  WDAT - Watchdog Action Table
///         Version 1
/// 
///  Conforms to \"Hardware Watchdog Timers Design Specification\",
///  Copyright 2006 Microsoft Corporation.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableWdat {
    pub header: FfiAcpiTableHeader,
    pub header_length: u32,
    pub pci_segment: u16,
    pub pci_bus: u8,
    pub pci_device: u8,
    pub pci_function: u8,
    pub reserved: [u8; 3usize],
    pub timer_period: u32,
    pub max_count: u32,
    pub min_count: u32,
    pub flags: u8,
    pub reserved2: [u8; 3usize],
    pub entries: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiWdatEntry {
    pub action: u8,
    pub instruction: u8,
    pub reserved: u16,
    pub register_region: AcpiGenericAddress,
    pub value: u32,
    pub mask: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiWdatActions {
    ACPI_WDAT_RESET = 1,
    ACPI_WDAT_GET_CURRENT_COUNTDOWN = 4,
    ACPI_WDAT_GET_COUNTDOWN = 5,
    ACPI_WDAT_SET_COUNTDOWN = 6,
    ACPI_WDAT_GET_RUNNING_STATE = 8,
    ACPI_WDAT_SET_RUNNING_STATE = 9,
    ACPI_WDAT_GET_STOPPED_STATE = 10,
    ACPI_WDAT_SET_STOPPED_STATE = 11,
    ACPI_WDAT_GET_REBOOT = 16,
    ACPI_WDAT_SET_REBOOT = 17,
    ACPI_WDAT_GET_SHUTDOWN = 18,
    ACPI_WDAT_SET_SHUTDOWN = 19,
    ACPI_WDAT_GET_STATUS = 32,
    ACPI_WDAT_SET_STATUS = 33,
    ACPI_WDAT_ACTION_RESERVED = 34,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiWdatInstructions {
    ACPI_WDAT_READ_VALUE = 0,
    ACPI_WDAT_READ_COUNTDOWN = 1,
    ACPI_WDAT_WRITE_VALUE = 2,
    ACPI_WDAT_WRITE_COUNTDOWN = 3,
    ACPI_WDAT_INSTRUCTION_RESERVED = 4,
    ACPI_WDAT_PRESERVE_REGISTER = 128,
}

///  WDDT - Watchdog Descriptor Table
///         Version 1
/// 
///  Conforms to \"Using the Intel ICH Family Watchdog Timer (WDT)\",
///  Version 001, September 2002
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableWddt {
    pub header: FfiAcpiTableHeader,
    pub spec_version: u16,
    pub table_version: u16,
    pub pci_vendor_id: u16,
    pub address: AcpiGenericAddress,
    pub max_count: u16,
    pub min_count: u16,
    pub period: u16,
    pub status: u16,
    pub capability: u16,
}

///  WDRT - Watchdog Resource Table
///         Version 1
/// 
///  Conforms to \"Watchdog Timer Hardware Requirements for Windows Server 2003\",
///  Version 1.01, August 28, 2006
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableWdrt {
    pub header: FfiAcpiTableHeader,
    pub control_register: AcpiGenericAddress,
    pub count_register: AcpiGenericAddress,
    pub pci_device_id: u16,
    pub pci_vendor_id: u16,
    pub pci_bus: u8,
    pub pci_device: u8,
    pub pci_function: u8,
    pub pci_segment: u8,
    pub max_count: u16,
    pub units: u8,
}

