use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField};

///  CEDT - CXL Early Discovery Table
///         Version 1
/// 
///  Conforms to the \"CXL Early Discovery Table\" (CXL 2.0)
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableCedt {
    pub Header: FfiAcpiTableHeader,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiCedtHeader {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiCedtType {
    Chbs = 0,
    Cfmws = 1,
    Reserved = 2,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiCedtChbs {
    pub Header: FfiAcpiCedtHeader,
    pub Uid: u32,
    pub CxlVersion: u32,
    pub Reserved: u32,
    pub Base: u64,
    pub Length: u64,
}

#[repr(C, packed)]
pub struct FfiAcpiCedtCfmws {
    pub Header: FfiAcpiCedtHeader,
    pub Reserved1: u32,
    pub BaseHpa: u64,
    pub WindowSize: u64,
    pub InterleaveWays: u8,
    pub InterleaveArithmetic: u8,
    pub Reserved2: u16,
    pub Granularity: u32,
    pub Restrictions: u16,
    pub QtgId: u16,
    InterleaveTargets: __IncompleteArrayField<u32>,
}