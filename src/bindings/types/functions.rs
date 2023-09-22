use crate::interface::status::AcpiStatus;

use super::{FfiAcpiHandle, FfiAcpiName, FfiAcpiPhysicalAddress, FfiAcpiResource, FfiAcpiString};

pub(crate) type FfiAcpiOsdHandler =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void) -> u32>;
pub(crate) type FfiAcpiOsdExecCallback =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void)>;
pub(crate) type FfiAcpiSciHandler =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void) -> u32>;
pub(crate) type FfiAcpiGblEventHandler = ::core::option::Option<
    unsafe extern "C" fn(
        EventType: u32,
        Device: FfiAcpiHandle,
        EventNumber: u32,
        Context: *mut ::core::ffi::c_void,
    ),
>;
pub(crate) type FfiAcpiEventHandler =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void) -> u32>;
pub(crate) type FfiAcpiGpeHandler = ::core::option::Option<
    unsafe extern "C" fn(
        GpeDevice: FfiAcpiHandle,
        GpeNumber: u32,
        Context: *mut ::core::ffi::c_void,
    ) -> u32,
>;
pub(crate) type FfiAcpiNotifyHandler = ::core::option::Option<
    unsafe extern "C" fn(Device: FfiAcpiHandle, Value: u32, Context: *mut ::core::ffi::c_void),
>;
pub(crate) type FfiAcpiObjectHandler = ::core::option::Option<
    unsafe extern "C" fn(Object: FfiAcpiHandle, Data: *mut ::core::ffi::c_void),
>;
pub(crate) type FfiAcpiInitHandler = ::core::option::Option<
    unsafe extern "C" fn(Object: FfiAcpiHandle, Function: u32) -> AcpiStatus,
>;
pub(crate) type FfiAcpiExceptionHandler = ::core::option::Option<
    unsafe extern "C" fn(
        AmlStatus: AcpiStatus,
        Name: FfiAcpiName,
        Opcode: u16,
        AmlOffset: u32,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type FfiAcpiTableHandler = ::core::option::Option<
    unsafe extern "C" fn(
        Event: u32,
        Table: *mut ::core::ffi::c_void,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type FfiAcpiAdrSpaceHandler = ::core::option::Option<
    unsafe extern "C" fn(
        Function: u32,
        Address: FfiAcpiPhysicalAddress,
        BitWidth: u32,
        Value: *mut u64,
        HandlerContext: *mut ::core::ffi::c_void,
        RegionContext: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type FfiAcpiAdrSpaceSetup = ::core::option::Option<
    unsafe extern "C" fn(
        RegionHandle: FfiAcpiHandle,
        Function: u32,
        HandlerContext: *mut ::core::ffi::c_void,
        RegionContext: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type FfiAcpiWalkCallback = ::core::option::Option<
    unsafe extern "C" fn(
        Object: FfiAcpiHandle,
        NestingLevel: u32,
        Context: *mut ::core::ffi::c_void,
        ReturnValue: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type FfiAcpiWalkResourceCallback = ::core::option::Option<
    unsafe extern "C" fn(
        Resource: *mut FfiAcpiResource,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type FfiAcpiInterfaceHandler = ::core::option::Option<
    unsafe extern "C" fn(InterfaceName: FfiAcpiString, Supported: u32) -> u32,
>;
