
#[doc = " PRMT - Platform Runtime Mechanism Table"]
#[doc = "        Version 1"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_prmt {
    pub Header: AcpiTableHeader,
}
#[doc = " PRMT - Platform Runtime Mechanism Table"]
#[doc = "        Version 1"]
#[doc = ""]
pub type ACPI_TABLE_PRMT = acpi_table_prmt;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_prmt_header {
    pub PlatformGuid: [u8; 16usize],
    pub ModuleInfoOffset: u32,
    pub ModuleInfoCount: u32,
}
pub type ACPI_TABLE_PRMT_HEADER = acpi_table_prmt_header;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_prmt_module_header {
    pub Revision: u16,
    pub Length: u16,
}
pub type ACPI_PRMT_MODULE_HEADER = acpi_prmt_module_header;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_prmt_module_info {
    pub Revision: u16,
    pub Length: u16,
    pub ModuleGuid: [u8; 16usize],
    pub MajorRev: u16,
    pub MinorRev: u16,
    pub HandlerInfoCount: u16,
    pub HandlerInfoOffset: u32,
    pub MmioListPointer: u64,
}
pub type ACPI_PRMT_MODULE_INFO = acpi_prmt_module_info;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_prmt_handler_info {
    pub Revision: u16,
    pub Length: u16,
    pub HandlerGuid: [u8; 16usize],
    pub HandlerAddress: u64,
    pub StaticDataBufferAddress: u64,
    pub AcpiParamBufferAddress: u64,
}
pub type ACPI_PRMT_HANDLER_INFO = acpi_prmt_handler_info;