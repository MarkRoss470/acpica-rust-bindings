use crate::bindings::types::FfiAcpiTableHeader;


///  PRMT - Platform Runtime Mechanism Table
///         Version 1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePrmt {
    pub Header: FfiAcpiTableHeader,
}
///  PRMT - Platform Runtime Mechanism Table
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePrmtHeader {
    pub PlatformGuid: [u8; 16usize],
    pub ModuleInfoOffset: u32,
    pub ModuleInfoCount: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPrmtModuleHeader {
    pub Revision: u16,
    pub Length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPrmtModuleInfo {
    pub Revision: u16,
    pub Length: u16,
    pub ModuleGuid: [u8; 16usize],
    pub MajorRev: u16,
    pub MinorRev: u16,
    pub HandlerInfoCount: u16,
    pub HandlerInfoOffset: u32,
    pub MmioListPointer: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPrmtHandlerInfo {
    pub Revision: u16,
    pub Length: u16,
    pub HandlerGuid: [u8; 16usize],
    pub HandlerAddress: u64,
    pub StaticDataBufferAddress: u64,
    pub AcpiParamBufferAddress: u64,
}
