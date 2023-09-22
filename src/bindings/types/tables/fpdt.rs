use crate::bindings::types::FfiAcpiTableHeader;

///  FPDT - Firmware Performance Data Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_fpdt {
    pub Header: FfiAcpiTableHeader,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_header {
    pub Type: u16,
    pub Length: u8,
    pub Revision: u8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiFpdtType {
    ACPI_FPDT_TYPE_BOOT = 0,
    ACPI_FPDT_TYPE_S3PERF = 1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_boot_pointer {
    pub Header: acpi_fpdt_header,
    pub Reserved: [u8; 4usize],
    pub Address: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_s3pt_pointer {
    pub Header: acpi_fpdt_header,
    pub Reserved: [u8; 4usize],
    pub Address: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_s3pt {
    pub Signature: [u8; 4usize],
    pub Length: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiS3ptType {
    ACPI_S3PT_TYPE_RESUME = 0,
    ACPI_S3PT_TYPE_SUSPEND = 1,
    ACPI_FPDT_BOOT_PERFORMANCE = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_s3pt_resume {
    pub Header: acpi_fpdt_header,
    pub ResumeCount: u32,
    pub FullResume: u64,
    pub AverageResume: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_s3pt_suspend {
    pub Header: acpi_fpdt_header,
    pub SuspendStart: u64,
    pub SuspendEnd: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_boot {
    pub Header: acpi_fpdt_header,
    pub Reserved: [u8; 4usize],
    pub ResetEnd: u64,
    pub LoadStart: u64,
    pub StartupStart: u64,
    pub ExitServicesEntry: u64,
    pub ExitServicesExit: u64,
}