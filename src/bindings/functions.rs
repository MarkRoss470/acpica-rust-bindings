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

    pub(crate) fn AcpiInstallInterface(InterfaceName: FfiAcpiString) -> AcpiStatus;

    pub(crate) fn AcpiRemoveInterface(InterfaceName: FfiAcpiString) -> AcpiStatus;

    pub(crate) fn AcpiUpdateInterfaces(Action: u8) -> AcpiStatus;

    pub(crate) fn AcpiCheckAddressRange(
        SpaceId: FfiAcpiAdtSpaceType,
        Address: FfiAcpiPhysicalAddress,
        Length: FfiAcpiSize,
        Warn: bool,
    ) -> u32;

    pub(crate) fn AcpiDecodePldBuffer(
        InBuffer: *mut u8,
        Length: FfiAcpiSize,
        ReturnBuffer: *mut *mut ACPI_PLD_INFO,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallTable(Address: FfiAcpiPhysicalAddress, Physical: bool) -> AcpiStatus;

    pub(crate) fn AcpiLoadTable(Table: *mut AcpiTableHeader, TableIdx: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiUnloadTable(TableIndex: u32) -> AcpiStatus;

    pub(crate) fn AcpiUnloadParentTable(Object: FfiAcpiHandle) -> AcpiStatus;

    pub(crate) fn AcpiLoadTables() -> AcpiStatus;

    pub(crate) fn AcpiReallocateRootTable() -> AcpiStatus;

    pub(crate) fn AcpiFindRootPointer(RsdpAddress: *mut FfiAcpiPhysicalAddress) -> AcpiStatus;

    pub(crate) fn AcpiGetTableHeader(
        Signature: FfiAcpiString,
        Instance: u32,
        OutTableHeader: *mut AcpiTableHeader,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetTable(
        Signature: FfiAcpiString,
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
        StartObject: FfiAcpiHandle,
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
        Object: FfiAcpiHandle,
        NameType: u32,
        RetPathPtr: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetHandle(
        Parent: FfiAcpiHandle,
        Pathname: FfiAcpiString,
        RetHandle: *mut FfiAcpiHandle,
    ) -> AcpiStatus;

    pub(crate) fn AcpiAttachData(
        Object: FfiAcpiHandle,
        Handler: ACPI_OBJECT_HANDLER,
        Data: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiDetachData(Object: FfiAcpiHandle, Handler: ACPI_OBJECT_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiGetData(
        Object: FfiAcpiHandle,
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
        Object: FfiAcpiHandle,
        Pathname: FfiAcpiString,
        ParameterObjects: *mut ACPI_OBJECT_LIST,
        ReturnObjectBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiEvaluateObjectTyped(
        Object: FfiAcpiHandle,
        Pathname: FfiAcpiString,
        ExternalParams: *mut ACPI_OBJECT_LIST,
        ReturnBuffer: *mut ACPI_BUFFER,
        ReturnType: AcpiObjectType,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetObjectInfo(
        Object: FfiAcpiHandle,
        ReturnBuffer: *mut *mut ACPI_DEVICE_INFO,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallMethod(Buffer: *mut u8) -> AcpiStatus;

    pub(crate) fn AcpiGetNextObject(
        Type: AcpiObjectType,
        Parent: FfiAcpiHandle,
        Child: FfiAcpiHandle,
        OutHandle: *mut FfiAcpiHandle,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetType(Object: FfiAcpiHandle, OutType: *mut AcpiObjectType) -> AcpiStatus;

    pub(crate) fn AcpiGetParent(Object: FfiAcpiHandle, OutHandle: *mut FfiAcpiHandle) -> AcpiStatus;

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
        GpeDevice: FfiAcpiHandle,
        GpeNumber: u32,
        Type: u32,
        Address: ACPI_GPE_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallGpeRawHandler(
        GpeDevice: FfiAcpiHandle,
        GpeNumber: u32,
        Type: u32,
        Address: ACPI_GPE_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveGpeHandler(
        GpeDevice: FfiAcpiHandle,
        GpeNumber: u32,
        Address: ACPI_GPE_HANDLER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallNotifyHandler(
        Device: FfiAcpiHandle,
        HandlerType: u32,
        Handler: ACPI_NOTIFY_HANDLER,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveNotifyHandler(
        Device: FfiAcpiHandle,
        HandlerType: u32,
        Handler: ACPI_NOTIFY_HANDLER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallAddressSpaceHandler(
        Device: FfiAcpiHandle,
        SpaceId: FfiAcpiAdtSpaceType,
        Handler: ACPI_ADR_SPACE_HANDLER,
        Setup: ACPI_ADR_SPACE_SETUP,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveAddressSpaceHandler(
        Device: FfiAcpiHandle,
        SpaceId: FfiAcpiAdtSpaceType,
        Handler: ACPI_ADR_SPACE_HANDLER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiInstallExceptionHandler(Handler: ACPI_EXCEPTION_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiInstallInterfaceHandler(Handler: ACPI_INTERFACE_HANDLER) -> AcpiStatus;

    pub(crate) fn AcpiAcquireGlobalLock(Timeout: u16, Handle: *mut u32) -> AcpiStatus;

    pub(crate) fn AcpiReleaseGlobalLock(Handle: u32) -> AcpiStatus;

    pub(crate) fn AcpiAcquireMutex(
        Handle: FfiAcpiHandle,
        Pathname: FfiAcpiString,
        Timeout: u16,
    ) -> AcpiStatus;

    pub(crate) fn AcpiReleaseMutex(Handle: FfiAcpiHandle, Pathname: FfiAcpiString) -> AcpiStatus;

    pub(crate) fn AcpiEnableEvent(Event: u32, Flags: u32) -> AcpiStatus;

    pub(crate) fn AcpiDisableEvent(Event: u32, Flags: u32) -> AcpiStatus;

    pub(crate) fn AcpiClearEvent(Event: u32) -> AcpiStatus;

    pub(crate) fn AcpiGetEventStatus(Event: u32, EventStatus: *mut FfiAcpiEventStatus) -> AcpiStatus;

    pub(crate) fn AcpiUpdateAllGpes() -> AcpiStatus;

    pub(crate) fn AcpiEnableGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiDisableGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiClearGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiSetGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32, Action: u8) -> AcpiStatus;

    pub(crate) fn AcpiFinishGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiMaskGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32, IsMasked: bool) -> AcpiStatus;

    pub(crate) fn AcpiMarkGpeForWake(GpeDevice: FfiAcpiHandle, GpeNumber: u32) -> AcpiStatus;

    pub(crate) fn AcpiSetupGpeForWake(
        ParentDevice: FfiAcpiHandle,
        GpeDevice: FfiAcpiHandle,
        GpeNumber: u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiSetGpeWakeMask(GpeDevice: FfiAcpiHandle, GpeNumber: u32, Action: u8) -> AcpiStatus;

    pub(crate) fn AcpiGetGpeStatus(
        GpeDevice: FfiAcpiHandle,
        GpeNumber: u32,
        EventStatus: *mut FfiAcpiEventStatus,
    ) -> AcpiStatus;

    pub(crate) fn AcpiDispatchGpe(GpeDevice: FfiAcpiHandle, GpeNumber: u32) -> u32;

    pub(crate) fn AcpiDisableAllGpes() -> AcpiStatus;

    pub(crate) fn AcpiEnableAllRuntimeGpes() -> AcpiStatus;

    pub(crate) fn AcpiEnableAllWakeupGpes() -> AcpiStatus;

    pub(crate) fn AcpiAnyGpeStatusSet() -> u32;

    pub(crate) fn AcpiGetGpeDevice(GpeIndex: u32, GpeDevice: *mut FfiAcpiHandle) -> AcpiStatus;

    pub(crate) fn AcpiInstallGpeBlock(
        GpeDevice: FfiAcpiHandle,
        GpeBlockAddress: *mut FfiAcpiGenericAddress,
        RegisterCount: u32,
        InterruptNumber: u32,
    ) -> AcpiStatus;

    pub(crate) fn AcpiRemoveGpeBlock(GpeDevice: FfiAcpiHandle) -> AcpiStatus;
}

extern "C" {
    pub(crate) fn AcpiGetVendorResource(
        Device: FfiAcpiHandle,
        Name: *mut i8,
        Uuid: *mut ACPI_VENDOR_UUID,
        RetBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetCurrentResources(Device: FfiAcpiHandle, RetBuffer: *mut ACPI_BUFFER)
        -> AcpiStatus;

    pub(crate) fn AcpiGetPossibleResources(
        Device: FfiAcpiHandle,
        RetBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiGetEventResources(
        DeviceHandle: FfiAcpiHandle,
        RetBuffer: *mut ACPI_BUFFER,
    ) -> AcpiStatus;

    pub(crate) fn AcpiWalkResourceBuffer(
        Buffer: *mut ACPI_BUFFER,
        UserFunction: ACPI_WALK_RESOURCE_CALLBACK,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiWalkResources(
        Device: FfiAcpiHandle,
        Name: *mut i8,
        UserFunction: ACPI_WALK_RESOURCE_CALLBACK,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus;

    pub(crate) fn AcpiSetCurrentResources(Device: FfiAcpiHandle, InBuffer: *mut ACPI_BUFFER) -> AcpiStatus;

    pub(crate) fn AcpiGetIrqRoutingTable(Device: FfiAcpiHandle, RetBuffer: *mut ACPI_BUFFER) -> AcpiStatus;

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
        PhysicalAddress: FfiAcpiPhysicalAddress,
        PhysicalAddress64: FfiAcpiPhysicalAddress,
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
