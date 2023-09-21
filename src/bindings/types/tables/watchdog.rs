use crate::{types::AcpiTableHeader, interface::AcpiGenericAddress};

///  WDAT - Watchdog Action Table
///         Version 1
/// 
///  Conforms to \"Hardware Watchdog Timers Design Specification\",
///  Copyright 2006 Microsoft Corporation.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_wdat {
    pub Header: AcpiTableHeader,
    pub HeaderLength: u32,
    pub PciSegment: u16,
    pub PciBus: u8,
    pub PciDevice: u8,
    pub PciFunction: u8,
    pub Reserved: [u8; 3usize],
    pub TimerPeriod: u32,
    pub MaxCount: u32,
    pub MinCount: u32,
    pub Flags: u8,
    pub Reserved2: [u8; 3usize],
    pub Entries: u32,
}
///  WDAT - Watchdog Action Table
///         Version 1
/// 
///  Conforms to \"Hardware Watchdog Timers Design Specification\",
///  Copyright 2006 Microsoft Corporation.
/// 
pub type ACPI_TABLE_WDAT = acpi_table_wdat;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_wdat_entry {
    pub Action: u8,
    pub Instruction: u8,
    pub Reserved: u16,
    pub RegisterRegion: AcpiGenericAddress,
    pub Value: u32,
    pub Mask: u32,
}
pub type ACPI_WDAT_ENTRY = acpi_wdat_entry;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiWdatActions {
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
pub enum AcpiWdatInstructions {
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
pub struct acpi_table_wddt {
    pub Header: AcpiTableHeader,
    pub SpecVersion: u16,
    pub TableVersion: u16,
    pub PciVendorId: u16,
    pub Address: AcpiGenericAddress,
    pub MaxCount: u16,
    pub MinCount: u16,
    pub Period: u16,
    pub Status: u16,
    pub Capability: u16,
}
///  WDDT - Watchdog Descriptor Table
///         Version 1
/// 
///  Conforms to \"Using the Intel ICH Family Watchdog Timer (WDT)\",
///  Version 001, September 2002
/// 
pub type ACPI_TABLE_WDDT = acpi_table_wddt;
///  WDRT - Watchdog Resource Table
///         Version 1
/// 
///  Conforms to \"Watchdog Timer Hardware Requirements for Windows Server 2003\",
///  Version 1.01, August 28, 2006
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_wdrt {
    pub Header: AcpiTableHeader,
    pub ControlRegister: AcpiGenericAddress,
    pub CountRegister: AcpiGenericAddress,
    pub PciDeviceId: u16,
    pub PciVendorId: u16,
    pub PciBus: u8,
    pub PciDevice: u8,
    pub PciFunction: u8,
    pub PciSegment: u8,
    pub MaxCount: u16,
    pub Units: u8,
}
///  WDRT - Watchdog Resource Table
///         Version 1
/// 
///  Conforms to \"Watchdog Timer Hardware Requirements for Windows Server 2003\",
///  Version 1.01, August 28, 2006
/// 
pub type ACPI_TABLE_WDRT = acpi_table_wdrt;
