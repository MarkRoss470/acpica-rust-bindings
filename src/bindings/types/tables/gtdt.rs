use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};


///  GTDT - Generic Timer Description Table (ACPI 5.1)
///         Version 2
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_gtdt {
    pub Header: FfiAcpiTableHeader,
    pub CounterBlockAddresss: u64,
    pub Reserved: u32,
    pub SecureEl1Interrupt: u32,
    pub SecureEl1Flags: u32,
    pub NonSecureEl1Interrupt: u32,
    pub NonSecureEl1Flags: u32,
    pub VirtualTimerInterrupt: u32,
    pub VirtualTimerFlags: u32,
    pub NonSecureEl2Interrupt: u32,
    pub NonSecureEl2Flags: u32,
    pub CounterReadBlockAddress: u64,
    pub PlatformTimerCount: u32,
    pub PlatformTimerOffset: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_gtdt_el2 {
    pub VirtualEL2TimerGsiv: u32,
    pub VirtualEL2TimerFlags: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_gtdt_header {
    pub Type: u8,
    pub Length: u16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiGtdtType {
    ACPI_GTDT_TYPE_TIMER_BLOCK = 0,
    ACPI_GTDT_TYPE_WATCHDOG = 1,
    ACPI_GTDT_TYPE_RESERVED = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_gtdt_timer_block {
    pub Header: acpi_gtdt_header,
    pub Reserved: u8,
    pub BlockAddress: u64,
    pub TimerCount: u32,
    pub TimerOffset: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_gtdt_timer_entry {
    pub FrameNumber: u8,
    pub Reserved: [u8; 3usize],
    pub BaseAddress: u64,
    pub El0BaseAddress: u64,
    pub TimerInterrupt: u32,
    pub TimerFlags: u32,
    pub VirtualTimerInterrupt: u32,
    pub VirtualTimerFlags: u32,
    pub CommonFlags: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_gtdt_watchdog {
    pub Header: acpi_gtdt_header,
    pub Reserved: u8,
    pub RefreshFrameAddress: u64,
    pub ControlFrameAddress: u64,
    pub TimerInterrupt: u32,
    pub TimerFlags: u32,
}