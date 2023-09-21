use crate::{types::AcpiTableHeader, interface::status::AcpiStatus};

use super::types::{*, object::AcpiObjectType};

extern "C" {
    pub(crate) fn AcpiInitializeTables(
        InitialStorage: *mut ACPI_TABLE_DESC,
        InitialTableCount: u32,
        AllowResize: bool,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInitializeSubsystem() -> AcpiStatus;

    pub(crate) fn AcpiEnableSubsystem(Flags: u32) -> AcpiStatus;

    pub(crate) fn AcpiInitializeObjects(Flags: u32) -> AcpiStatus;

    pub(crate) fn AcpiTerminate() -> AcpiStatus;

    pub(crate) fn AcpiEnable() -> AcpiStatus;

    pub(crate) fn AcpiDisable() -> AcpiStatus;

    pub(crate) fn AcpiSubsystemStatus() -> AcpiStatus;

    pub(crate) fn AcpiGetSystemInfo(RetBuffer: *mut ACPI_BUFFER) -> AcpiStatus;

    pub(crate) fn AcpiGetStatistics(Stats: *mut ACPI_STATISTICS) -> AcpiStatus;

    pub(crate) fn AcpiFormatException(Exception: AcpiStatus) -> *const i8;

    pub(crate) fn AcpiPurgeCachedObjects() -> AcpiStatus;

    pub(crate) fn AcpiInstallInterface(InterfaceName: ACPI_STRING) -> AcpiStatus;

    pub(crate) fn AcpiRemoveInterface(InterfaceName: ACPI_STRING) -> AcpiStatus;

    pub(crate) fn AcpiUpdateInterfaces(Action: u8) -> AcpiStatus;

    pub(crate) fn AcpiCheckAddressRange(
        SpaceId: ACPI_ADR_SPACE_TYPE,
        Address: ACPI_PHYSICAL_ADDRESS,
        Length: ACPI_SIZE,
        Warn: bool,
    ) -> u32;

    pub(crate) fn AcpiDecodePldBuffer(
        InBuffer: *mut u8,
        Length: ACPI_SIZE,
        ReturnBuffer: *mut *mut ACPI_PLD_INFO,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallTable(Address: ACPI_PHYSICAL_ADDRESS, Physical: bool) -> AcpiStatus;

    pub(crate) fn AcpiLoadTable(Table: *mut AcpiTableHeader, TableIdx: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiUnloadTable(TableIndex: u32) -> AcpiStatus;

    pub(crate) fn AcpiUnloadParentTable(Object: ACPI_HANDLE) -> AcpiStatus;

    pub(crate) fn AcpiLoadTables() -> AcpiStatus;

    pub(crate) fn AcpiReallocateRootTable() -> AcpiStatus;

    pub(crate) fn AcpiFindRootPointer(RsdpAddress: *mut ACPI_PHYSICAL_ADDRESS) -> AcpiStatus;

    pub(crate) fn AcpiGetTableHeader(
        Signature: ACPI_STRING,
        Instance: u32,
        OutTableHeader: *mut AcpiTableHeader,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetTable(
        Signature: ACPI_STRING,
        Instance: u32,
        OutTable: *mut *mut AcpiTableHeader,
    ) -> AcpiStatus;

    pub(crate) fn AcpiPutTable(Table: *mut AcpiTableHeader);

    pub(crate) fn AcpiGetTableByIndex(
        TableIndex: u32,
        OutTable: *mut *mut AcpiTableHeader,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallTableHandler(
        Handler: ACPI_TABLE_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveTableHandler(Handler: ACPI_TABLE_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiWalkNamespace(
        Type: AcpiObjectType,
        StartObject: ACPI_HANDLE,
        MaxDepth: u32,
        DescendingCallback: ACPI_WALK_CALLBACK,
        AscendingCallback: ACPI_WALK_CALLBACK,
        Context: *mut ::core::ffi::c_void,
        ReturnValue: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetDevices(
        HID: *mut i8,
        UserFunction: ACPI_WALK_CALLBACK,
        Context: *mut ::core::ffi::c_void,
        ReturnValue: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetName(
        Object: ACPI_HANDLE,
        NameType: u32,
        RetPathPtr: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetHandle(
        Parent: ACPI_HANDLE,
        Pathname: ACPI_STRING,
        RetHandle: *mut ACPI_HANDLE,
    ) -> AcpiStatus;

    pub(crate) fn AcpiAttachData(
        Object: ACPI_HANDLE,
        Handler: ACPI_OBJECT_HANDLER,
        Data: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiDetachData(Object: ACPI_HANDLE, Handler: ACPI_OBJECT_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiGetData(
        Object: ACPI_HANDLE,
        Handler: ACPI_OBJECT_HANDLER,
        Data: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiDebugTrace(
        Name: *const i8,
        DebugLevel: u32,
        DebugLayer: u32,
        Flags: u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiEvaluateObject(
        Object: ACPI_HANDLE,
        Pathname: ACPI_STRING,
        ParameterObjects: *mut ACPI_OBJECT_LIST,
        ReturnObjectBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiEvaluateObjectTyped(
        Object: ACPI_HANDLE,
        Pathname: ACPI_STRING,
        ExternalParams: *mut ACPI_OBJECT_LIST,
        ReturnBuffer: *mut ACPI_BUFFER,
        ReturnType: AcpiObjectType,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetObjectInfo(
        Object: ACPI_HANDLE,
        ReturnBuffer: *mut *mut ACPI_DEVICE_INFO,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallMethod(Buffer: *mut u8) -> AcpiStatus;

    pub(crate) fn AcpiGetNextObject(
        Type: AcpiObjectType,
        Parent: ACPI_HANDLE,
        Child: ACPI_HANDLE,
        OutHandle: *mut ACPI_HANDLE,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetType(Object: ACPI_HANDLE, OutType: *mut AcpiObjectType) -> AcpiStatus;

    pub(crate) fn AcpiGetParent(Object: ACPI_HANDLE, OutHandle: *mut ACPI_HANDLE) -> AcpiStatus;

    pub(crate) fn AcpiInstallInitializationHandler(
        Handler: ACPI_INIT_HANDLER,
        Function: u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallSciHandler(
        Address: ACPI_SCI_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveSciHandler(Address: ACPI_SCI_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiInstallGlobalEventHandler(
        Handler: ACPI_GBL_EVENT_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallFixedEventHandler(
        AcpiEvent: u32,
        Handler: ACPI_EVENT_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveFixedEventHandler(AcpiEvent: u32, Handler: ACPI_EVENT_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiInstallGpeHandler(
        GpeDevice: ACPI_HANDLE,
        GpeNumber: u32,
        Type: u32,
        Address: ACPI_GPE_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallGpeRawHandler(
        GpeDevice: ACPI_HANDLE,
        GpeNumber: u32,
        Type: u32,
        Address: ACPI_GPE_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveGpeHandler(
        GpeDevice: ACPI_HANDLE,
        GpeNumber: u32,
        Address: ACPI_GPE_HANDLER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallNotifyHandler(
        Device: ACPI_HANDLE,
        HandlerType: u32,
        Handler: ACPI_NOTIFY_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveNotifyHandler(
        Device: ACPI_HANDLE,
        HandlerType: u32,
        Handler: ACPI_NOTIFY_HANDLER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallAddressSpaceHandler(
        Device: ACPI_HANDLE,
        SpaceId: ACPI_ADR_SPACE_TYPE,
        Handler: ACPI_ADR_SPACE_HANDLER,
        Setup: ACPI_ADR_SPACE_SETUP,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveAddressSpaceHandler(
        Device: ACPI_HANDLE,
        SpaceId: ACPI_ADR_SPACE_TYPE,
        Handler: ACPI_ADR_SPACE_HANDLER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallExceptionHandler(Handler: ACPI_EXCEPTION_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiInstallInterfaceHandler(Handler: ACPI_INTERFACE_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiAcquireGlobalLock(Timeout: u16, Handle: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiReleaseGlobalLock(Handle: u32) -> AcpiStatus;

    pub(crate) fn AcpiAcquireMutex(
        Handle: ACPI_HANDLE,
        Pathname: ACPI_STRING,
        Timeout: u16,
    ) -> AcpiStatus;

    pub(crate) fn AcpiReleaseMutex(Handle: ACPI_HANDLE, Pathname: ACPI_STRING) -> AcpiStatus;

    pub(crate) fn AcpiEnableEvent(Event: u32, Flags: u32) -> AcpiStatus;

    pub(crate) fn AcpiDisableEvent(Event: u32, Flags: u32) -> AcpiStatus;

    pub(crate) fn AcpiClearEvent(Event: u32) -> AcpiStatus;

    pub(crate) fn AcpiGetEventStatus(Event: u32, EventStatus: *mut ACPI_EVENT_STATUS) -> AcpiStatus;

    pub(crate) fn AcpiUpdateAllGpes() -> AcpiStatus;

    pub(crate) fn AcpiEnableGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiDisableGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiClearGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiSetGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32, Action: u8) -> AcpiStatus;

    pub(crate) fn AcpiFinishGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiMaskGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32, IsMasked: bool) -> AcpiStatus;

    pub(crate) fn AcpiMarkGpeForWake(GpeDevice: ACPI_HANDLE, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiSetupGpeForWake(
        ParentDevice: ACPI_HANDLE,
        GpeDevice: ACPI_HANDLE,
        GpeNumber: u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiSetGpeWakeMask(GpeDevice: ACPI_HANDLE, GpeNumber: u32, Action: u8) -> AcpiStatus;

    pub(crate) fn AcpiGetGpeStatus(
        GpeDevice: ACPI_HANDLE,
        GpeNumber: u32,
        EventStatus: *mut ACPI_EVENT_STATUS,
    ) -> AcpiStatus;

    pub(crate) fn AcpiDispatchGpe(GpeDevice: ACPI_HANDLE, GpeNumber: u32) -> u32;

    pub(crate) fn AcpiDisableAllGpes() -> AcpiStatus;

    pub(crate) fn AcpiEnableAllRuntimeGpes() -> AcpiStatus;

    pub(crate) fn AcpiEnableAllWakeupGpes() -> AcpiStatus;

    pub(crate) fn AcpiAnyGpeStatusSet() -> u32;

    pub(crate) fn AcpiGetGpeDevice(GpeIndex: u32, GpeDevice: *mut ACPI_HANDLE) -> AcpiStatus;

    pub(crate) fn AcpiInstallGpeBlock(
        GpeDevice: ACPI_HANDLE,
        GpeBlockAddress: *mut FfiAcpiGenericAddress,
        RegisterCount: u32,
        InterruptNumber: u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveGpeBlock(GpeDevice: ACPI_HANDLE) -> AcpiStatus;
}

extern "C" {
    pub(crate) fn AcpiGetVendorResource(
        Device: ACPI_HANDLE,
        Name: *mut i8,
        Uuid: *mut ACPI_VENDOR_UUID,
        RetBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetCurrentResources(Device: ACPI_HANDLE, RetBuffer: *mut ACPI_BUFFER)
        -> AcpiStatus;

    pub(crate) fn AcpiGetPossibleResources(
        Device: ACPI_HANDLE,
        RetBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetEventResources(
        DeviceHandle: ACPI_HANDLE,
        RetBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiWalkResourceBuffer(
        Buffer: *mut ACPI_BUFFER,
        UserFunction: ACPI_WALK_RESOURCE_CALLBACK,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiWalkResources(
        Device: ACPI_HANDLE,
        Name: *mut i8,
        UserFunction: ACPI_WALK_RESOURCE_CALLBACK,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiSetCurrentResources(Device: ACPI_HANDLE, InBuffer: *mut ACPI_BUFFER) -> AcpiStatus;

    pub(crate) fn AcpiGetIrqRoutingTable(Device: ACPI_HANDLE, RetBuffer: *mut ACPI_BUFFER) -> AcpiStatus;

    pub(crate) fn AcpiResourceToAddress64(
        Resource: *mut ACPI_RESOURCE,
        Out: *mut ACPI_RESOURCE_ADDRESS64,
    ) -> AcpiStatus;

    pub(crate) fn AcpiBufferToResource(
        AmlBuffer: *mut u8,
        AmlBufferLength: u16,
        ResourcePtr: *mut *mut ACPI_RESOURCE,
    ) -> AcpiStatus;

    pub(crate) fn AcpiReset() -> AcpiStatus;

    pub(crate) fn AcpiRead(Value: *mut u64, Reg: *mut FfiAcpiGenericAddress) -> AcpiStatus;

    pub(crate) fn AcpiWrite(Value: u64, Reg: *mut FfiAcpiGenericAddress) -> AcpiStatus;

    pub(crate) fn AcpiReadBitRegister(RegisterId: u32, ReturnValue: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiWriteBitRegister(RegisterId: u32, Value: u32) -> AcpiStatus;

    pub(crate) fn AcpiGetSleepTypeData(
        SleepState: u8,
        Slp_TypA: *mut u8,
        Slp_TypB: *mut u8,
    ) -> AcpiStatus;

    pub(crate) fn AcpiEnterSleepStatePrep(SleepState: u8) -> AcpiStatus;

    pub(crate) fn AcpiEnterSleepState(SleepState: u8) -> AcpiStatus;

    pub(crate) fn AcpiEnterSleepStateS4bios() -> AcpiStatus;

    pub(crate) fn AcpiLeaveSleepStatePrep(SleepState: u8) -> AcpiStatus;

    pub(crate) fn AcpiLeaveSleepState(SleepState: u8) -> AcpiStatus;

    pub(crate) fn AcpiSetFirmwareWakingVector(
        PhysicalAddress: ACPI_PHYSICAL_ADDRESS,
        PhysicalAddress64: ACPI_PHYSICAL_ADDRESS,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetTimerResolution(Resolution: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiGetTimer(Ticks: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiGetTimerDuration(
        StartTicks: u32,
        EndTicks: u32,
        TimeElapsed: *mut u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiError(ModuleName: *const i8, LineNumber: u32, Format: *const i8, ...);

    pub(crate) fn AcpiException(
        ModuleName: *const i8,
        LineNumber: u32,
        Status: AcpiStatus,
        Format: *const i8,
        ...
    );

    pub(crate) fn AcpiWarning(ModuleName: *const i8, LineNumber: u32, Format: *const i8, ...);

    pub(crate) fn AcpiInfo(Format: *const i8, ...);

    pub(crate) fn AcpiBiosError(ModuleName: *const i8, LineNumber: u32, Format: *const i8, ...);

    pub(crate) fn AcpiBiosException(
        ModuleName: *const i8,
        LineNumber: u32,
        Status: AcpiStatus,
        Format: *const i8,
        ...
    );

    pub(crate) fn AcpiBiosWarning(ModuleName: *const i8, LineNumber: u32, Format: *const i8, ...);

    pub(crate) fn AcpiInitializeDebugger() -> AcpiStatus;

    pub(crate) fn AcpiTerminateDebugger();

    pub(crate) fn AcpiRunDebugger(BatchBuffer: *mut i8);

    pub(crate) fn AcpiSetDebuggerThreadId(ThreadId: u64);
}
