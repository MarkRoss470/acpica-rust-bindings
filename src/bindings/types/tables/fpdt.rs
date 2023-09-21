#[doc = " FPDT - Firmware Performance Data Table (ACPI 5.0)"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_fpdt {
    pub Header: AcpiTableHeader,
}
#[doc = " FPDT - Firmware Performance Data Table (ACPI 5.0)"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_FPDT = acpi_table_fpdt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_header {
    pub Type: u16,
    pub Length: u8,
    pub Revision: u8,
}
pub type ACPI_FPDT_HEADER = acpi_fpdt_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiFpdtType {
    ACPI_FPDT_TYPE_BOOT = 0,
    ACPI_FPDT_TYPE_S3PERF = 1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_boot_pointer {
    pub Header: ACPI_FPDT_HEADER,
    pub Reserved: [u8; 4usize],
    pub Address: u64,
}
pub type ACPI_FPDT_BOOT_POINTER = acpi_fpdt_boot_pointer;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_s3pt_pointer {
    pub Header: ACPI_FPDT_HEADER,
    pub Reserved: [u8; 4usize],
    pub Address: u64,
}
pub type ACPI_FPDT_S3PT_POINTER = acpi_fpdt_s3pt_pointer;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_s3pt {
    pub Signature: [u8; 4usize],
    pub Length: u32,
}
pub type ACPI_TABLE_S3PT = acpi_table_s3pt;
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
    pub Header: ACPI_FPDT_HEADER,
    pub ResumeCount: u32,
    pub FullResume: u64,
    pub AverageResume: u64,
}
pub type ACPI_S3PT_RESUME = acpi_s3pt_resume;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_s3pt_suspend {
    pub Header: ACPI_FPDT_HEADER,
    pub SuspendStart: u64,
    pub SuspendEnd: u64,
}
pub type ACPI_S3PT_SUSPEND = acpi_s3pt_suspend;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_fpdt_boot {
    pub Header: ACPI_FPDT_HEADER,
    pub Reserved: [u8; 4usize],
    pub ResetEnd: u64,
    pub LoadStart: u64,
    pub StartupStart: u64,
    pub ExitServicesEntry: u64,
    pub ExitServicesExit: u64,
}
pub type ACPI_FPDT_BOOT = acpi_fpdt_boot;