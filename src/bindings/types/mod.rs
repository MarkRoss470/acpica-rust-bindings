pub(crate) mod tables;
pub mod object;

// pub(crate) use tables::*;
use tables::FfiAcpiTableHeader;

use crate::interface::status::AcpiStatus;

use self::object::{FfiAcpiObject, AcpiObjectType};

#[repr(C)]
#[derive(Default)]
pub(crate) struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub(crate) const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub(crate) unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub(crate) unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub(crate) type ACPI_NATIVE_INT = core::ffi::c_int;
pub(crate) type ACPI_SIZE = u64;
pub(crate) type ACPI_IO_ADDRESS = u64;
pub(crate) type ACPI_PHYSICAL_ADDRESS = u64;
pub(crate) type ACPI_NAME = u32;
pub(crate) type ACPI_STRING = *mut i8;
pub(crate) type ACPI_HANDLE = *mut ::core::ffi::c_void;
pub(crate) type ACPI_OWNER_ID = u16;
pub(crate) type ACPI_INTEGER = u64;
pub(crate) type ACPI_EVENT_TYPE = u32;
pub(crate) type ACPI_EVENT_STATUS = u32;
pub(crate) type ACPI_ADR_SPACE_TYPE = u8;
pub(crate) type ACPI_SLEEP_FUNCTION =
    ::core::option::Option<unsafe extern "C" fn(SleepState: u8) -> AcpiStatus>;

#[doc = " GAS - Generic Address Structure (ACPI 2.0+)"]
#[doc = ""]
#[doc = " Note: Since this structure is used in the ACPI tables, it is byte aligned."]
#[doc = " If misaligned access is not supported by the hardware, accesses to the"]
#[doc = " 64-bit Address field must be performed with care."]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiGenericAddress {
    pub(crate) SpaceId: u8,
    pub(crate) BitWidth: u8,
    pub(crate) BitOffset: u8,
    pub(crate) AccessWidth: u8,
    pub(crate) Address: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiSleepFunctions {
    pub(crate) LegacyFunction: ACPI_SLEEP_FUNCTION,
    pub(crate) ExtendedFunction: ACPI_SLEEP_FUNCTION,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_object_list {
    pub(crate) Count: u32,
    pub(crate) Pointer: *mut FfiAcpiObject,
}
pub(crate) type ACPI_OBJECT_LIST = acpi_object_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_buffer {
    pub(crate) Length: ACPI_SIZE,
    pub(crate) Pointer: *mut ::core::ffi::c_void,
}
pub(crate) type ACPI_BUFFER = acpi_buffer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_predefined_names {
    pub(crate) Name: *const i8,
    pub(crate) Type: u8,
    pub(crate) Val: *mut i8,
}
pub(crate) type ACPI_PREDEFINED_NAMES = acpi_predefined_names;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_system_info {
    pub(crate) AcpiCaVersion: u32,
    pub(crate) Flags: u32,
    pub(crate) TimerResolution: u32,
    pub(crate) Reserved1: u32,
    pub(crate) Reserved2: u32,
    pub(crate) DebugLevel: u32,
    pub(crate) DebugLayer: u32,
}
pub(crate) type ACPI_SYSTEM_INFO = acpi_system_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_statistics {
    pub(crate) SciCount: u32,
    pub(crate) GpeCount: u32,
    pub(crate) FixedEventCount: [u32; 5usize],
    pub(crate) MethodCount: u32,
}
pub(crate) type ACPI_STATISTICS = acpi_statistics;
pub(crate) type ACPI_OSD_HANDLER =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void) -> u32>;
pub(crate) type ACPI_OSD_EXEC_CALLBACK =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void)>;
pub(crate) type ACPI_SCI_HANDLER =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void) -> u32>;
pub(crate) type ACPI_GBL_EVENT_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(
        EventType: u32,
        Device: ACPI_HANDLE,
        EventNumber: u32,
        Context: *mut ::core::ffi::c_void,
    ),
>;
pub(crate) type ACPI_EVENT_HANDLER =
    ::core::option::Option<unsafe extern "C" fn(Context: *mut ::core::ffi::c_void) -> u32>;
pub(crate) type ACPI_GPE_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(
        GpeDevice: ACPI_HANDLE,
        GpeNumber: u32,
        Context: *mut ::core::ffi::c_void,
    ) -> u32,
>;
pub(crate) type ACPI_NOTIFY_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(Device: ACPI_HANDLE, Value: u32, Context: *mut ::core::ffi::c_void),
>;
pub(crate) type ACPI_OBJECT_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(Object: ACPI_HANDLE, Data: *mut ::core::ffi::c_void),
>;
pub(crate) type ACPI_INIT_HANDLER =
    ::core::option::Option<unsafe extern "C" fn(Object: ACPI_HANDLE, Function: u32) -> AcpiStatus>;
pub(crate) type ACPI_EXCEPTION_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(
        AmlStatus: AcpiStatus,
        Name: ACPI_NAME,
        Opcode: u16,
        AmlOffset: u32,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type ACPI_TABLE_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(
        Event: u32,
        Table: *mut ::core::ffi::c_void,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type ACPI_ADR_SPACE_HANDLER = ::core::option::Option<
    unsafe extern "C" fn(
        Function: u32,
        Address: ACPI_PHYSICAL_ADDRESS,
        BitWidth: u32,
        Value: *mut u64,
        HandlerContext: *mut ::core::ffi::c_void,
        RegionContext: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_connection_info {
    pub(crate) Connection: *mut u8,
    pub(crate) Length: u16,
    pub(crate) AccessLength: u8,
}
pub(crate) type ACPI_CONNECTION_INFO = acpi_connection_info;
pub(crate) type ACPI_ADR_SPACE_SETUP = ::core::option::Option<
    unsafe extern "C" fn(
        RegionHandle: ACPI_HANDLE,
        Function: u32,
        HandlerContext: *mut ::core::ffi::c_void,
        RegionContext: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type ACPI_WALK_CALLBACK = ::core::option::Option<
    unsafe extern "C" fn(
        Object: ACPI_HANDLE,
        NestingLevel: u32,
        Context: *mut ::core::ffi::c_void,
        ReturnValue: *mut *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
#[doc = " ACPICA public interface prototypes"]
#[doc = ""]
pub(crate) type ACPI_WALK_RESOURCE_CALLBACK = ::core::option::Option<
    unsafe extern "C" fn(
        Resource: *mut ACPI_RESOURCE,
        Context: *mut ::core::ffi::c_void,
    ) -> AcpiStatus,
>;
pub(crate) type ACPI_INTERFACE_HANDLER =
    ::core::option::Option<unsafe extern "C" fn(InterfaceName: ACPI_STRING, Supported: u32) -> u32>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_pnp_device_id {
    pub(crate) Length: u32,
    pub(crate) String: *mut i8,
}
pub(crate) type ACPI_PNP_DEVICE_ID = acpi_pnp_device_id;
#[repr(C)]
#[derive(Debug)]
pub(crate) struct acpi_pnp_device_id_list {
    pub(crate) Count: u32,
    pub(crate) ListSize: u32,
    pub(crate) Ids: __IncompleteArrayField<ACPI_PNP_DEVICE_ID>,
}
pub(crate) type ACPI_PNP_DEVICE_ID_LIST = acpi_pnp_device_id_list;
#[repr(C)]
#[derive(Debug)]
pub(crate) struct acpi_device_info {
    pub(crate) InfoSize: u32,
    pub(crate) Name: u32,
    pub(crate) Type: AcpiObjectType,
    pub(crate) ParamCount: u8,
    pub(crate) Valid: u16,
    pub(crate) Flags: u8,
    pub(crate) HighestDstates: [u8; 4usize],
    pub(crate) LowestDstates: [u8; 5usize],
    pub(crate) Address: u64,
    pub(crate) HardwareId: ACPI_PNP_DEVICE_ID,
    pub(crate) UniqueId: ACPI_PNP_DEVICE_ID,
    pub(crate) ClassCode: ACPI_PNP_DEVICE_ID,
    pub(crate) CompatibleIdList: ACPI_PNP_DEVICE_ID_LIST,
}
pub(crate) type ACPI_DEVICE_INFO = acpi_device_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_pci_id {
    pub(crate) Segment: u16,
    pub(crate) Bus: u16,
    pub(crate) Device: u16,
    pub(crate) Function: u16,
}
pub(crate) type ACPI_PCI_ID = acpi_pci_id;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_mem_mapping {
    pub(crate) PhysicalAddress: ACPI_PHYSICAL_ADDRESS,
    pub(crate) LogicalAddress: *mut u8,
    pub(crate) Length: ACPI_SIZE,
    pub(crate) NextMm: *mut acpi_mem_mapping,
}
pub(crate) type ACPI_MEM_MAPPING = acpi_mem_mapping;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_mem_space_context {
    pub(crate) Length: u32,
    pub(crate) Address: ACPI_PHYSICAL_ADDRESS,
    pub(crate) CurMm: *mut ACPI_MEM_MAPPING,
    pub(crate) FirstMm: *mut ACPI_MEM_MAPPING,
}
pub(crate) type ACPI_MEM_SPACE_CONTEXT = acpi_mem_space_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_memory_list {
    pub(crate) ListName: *const i8,
    pub(crate) ListHead: *mut ::core::ffi::c_void,
    pub(crate) ObjectSize: u16,
    pub(crate) MaxDepth: u16,
    pub(crate) CurrentDepth: u16,
}
pub(crate) type ACPI_MEMORY_LIST = acpi_memory_list;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum ACPI_TRACE_EVENT_TYPE {
    ACPI_TRACE_AML_METHOD = 0,
    ACPI_TRACE_AML_OPCODE = 1,
    ACPI_TRACE_AML_REGION = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_exception_info {
    pub(crate) Name: *mut i8,
}
pub(crate) type ACPI_EXCEPTION_INFO = acpi_exception_info;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum AcpiPreferredPmProfiles {
    PM_UNSPECIFIED = 0,
    PM_DESKTOP = 1,
    PM_MOBILE = 2,
    PM_WORKSTATION = 3,
    PM_ENTERPRISE_SERVER = 4,
    PM_SOHO_SERVER = 5,
    PM_APPLIANCE_PC = 6,
    PM_PERFORMANCE_SERVER = 7,
    PM_TABLET = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union acpi_name_union {
    pub(crate) Integer: u32,
    pub(crate) Ascii: [i8; 4usize],
}
pub(crate) type ACPI_NAME_UNION = acpi_name_union;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_table_desc {
    pub(crate) Address: ACPI_PHYSICAL_ADDRESS,
    pub(crate) Pointer: *mut FfiAcpiTableHeader,
    pub(crate) Length: u32,
    pub(crate) Signature: ACPI_NAME_UNION,
    pub(crate) OwnerId: ACPI_OWNER_ID,
    pub(crate) Flags: u8,
    pub(crate) ValidationCount: u16,
}
pub(crate) type ACPI_TABLE_DESC = acpi_table_desc;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_whea_header {
    pub(crate) Action: u8,
    pub(crate) Instruction: u8,
    pub(crate) Flags: u8,
    pub(crate) Reserved: u8,
    pub(crate) RegisterRegion: FfiAcpiGenericAddress,
    pub(crate) Value: u64,
    pub(crate) Mask: u64,
}
pub(crate) type ACPI_WHEA_HEADER = acpi_whea_header;

pub(crate) type ACPI_RS_LENGTH = u16;
pub(crate) type ACPI_RSDESC_SIZE = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_uuid {
    pub(crate) Data: [u8; 16usize],
}
pub(crate) type ACPI_UUID = acpi_uuid;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_vendor_uuid {
    pub(crate) Subtype: u8,
    pub(crate) Data: [u8; 16usize],
}
pub(crate) type ACPI_VENDOR_UUID = acpi_vendor_uuid;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_irq {
    pub(crate) DescriptorLength: u8,
    pub(crate) Triggering: u8,
    pub(crate) Polarity: u8,
    pub(crate) Shareable: u8,
    pub(crate) WakeCapable: u8,
    pub(crate) InterruptCount: u8,
    pub(crate) Interrupts: [u8; 1usize],
}
pub(crate) type ACPI_RESOURCE_IRQ = acpi_resource_irq;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_dma {
    pub(crate) Type: u8,
    pub(crate) BusMaster: u8,
    pub(crate) Transfer: u8,
    pub(crate) ChannelCount: u8,
    pub(crate) Channels: [u8; 1usize],
}
pub(crate) type ACPI_RESOURCE_DMA = acpi_resource_dma;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_start_dependent {
    pub(crate) DescriptorLength: u8,
    pub(crate) CompatibilityPriority: u8,
    pub(crate) PerformanceRobustness: u8,
}
pub(crate) type ACPI_RESOURCE_START_DEPENDENT = acpi_resource_start_dependent;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_io {
    pub(crate) IoDecode: u8,
    pub(crate) Alignment: u8,
    pub(crate) AddressLength: u8,
    pub(crate) Minimum: u16,
    pub(crate) Maximum: u16,
}
pub(crate) type ACPI_RESOURCE_IO = acpi_resource_io;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_fixed_io {
    pub(crate) Address: u16,
    pub(crate) AddressLength: u8,
}
pub(crate) type ACPI_RESOURCE_FIXED_IO = acpi_resource_fixed_io;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_fixed_dma {
    pub(crate) RequestLines: u16,
    pub(crate) Channels: u16,
    pub(crate) Width: u8,
}
pub(crate) type ACPI_RESOURCE_FIXED_DMA = acpi_resource_fixed_dma;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_vendor {
    pub(crate) ByteLength: u16,
    pub(crate) ByteData: [u8; 1usize],
}
pub(crate) type ACPI_RESOURCE_VENDOR = acpi_resource_vendor;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_vendor_typed {
    pub(crate) ByteLength: u16,
    pub(crate) UuidSubtype: u8,
    pub(crate) Uuid: [u8; 16usize],
    pub(crate) ByteData: [u8; 1usize],
}
pub(crate) type ACPI_RESOURCE_VENDOR_TYPED = acpi_resource_vendor_typed;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_end_tag {
    pub(crate) Checksum: u8,
}
pub(crate) type ACPI_RESOURCE_END_TAG = acpi_resource_end_tag;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_memory24 {
    pub(crate) WriteProtect: u8,
    pub(crate) Minimum: u16,
    pub(crate) Maximum: u16,
    pub(crate) Alignment: u16,
    pub(crate) AddressLength: u16,
}
pub(crate) type ACPI_RESOURCE_MEMORY24 = acpi_resource_memory24;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_memory32 {
    pub(crate) WriteProtect: u8,
    pub(crate) Minimum: u32,
    pub(crate) Maximum: u32,
    pub(crate) Alignment: u32,
    pub(crate) AddressLength: u32,
}
pub(crate) type ACPI_RESOURCE_MEMORY32 = acpi_resource_memory32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_fixed_memory32 {
    pub(crate) WriteProtect: u8,
    pub(crate) Address: u32,
    pub(crate) AddressLength: u32,
}
pub(crate) type ACPI_RESOURCE_FIXED_MEMORY32 = acpi_resource_fixed_memory32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_memory_attribute {
    pub(crate) WriteProtect: u8,
    pub(crate) Caching: u8,
    pub(crate) RangeType: u8,
    pub(crate) Translation: u8,
}
pub(crate) type ACPI_MEMORY_ATTRIBUTE = acpi_memory_attribute;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_io_attribute {
    pub(crate) RangeType: u8,
    pub(crate) Translation: u8,
    pub(crate) TranslationType: u8,
    pub(crate) Reserved1: u8,
}
pub(crate) type ACPI_IO_ATTRIBUTE = acpi_io_attribute;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union acpi_resource_attribute {
    pub(crate) Mem: ACPI_MEMORY_ATTRIBUTE,
    pub(crate) Io: ACPI_IO_ATTRIBUTE,
    pub(crate) TypeSpecific: u8,
}
pub(crate) type ACPI_RESOURCE_ATTRIBUTE = acpi_resource_attribute;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_label {
    pub(crate) StringLength: u16,
    pub(crate) StringPtr: *mut i8,
}
pub(crate) type ACPI_RESOURCE_LABEL = acpi_resource_label;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_source {
    pub(crate) Index: u8,
    pub(crate) StringLength: u16,
    pub(crate) StringPtr: *mut i8,
}
pub(crate) type ACPI_RESOURCE_SOURCE = acpi_resource_source;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_address16_attribute {
    pub(crate) Granularity: u16,
    pub(crate) Minimum: u16,
    pub(crate) Maximum: u16,
    pub(crate) TranslationOffset: u16,
    pub(crate) AddressLength: u16,
}
pub(crate) type ACPI_ADDRESS16_ATTRIBUTE = acpi_address16_attribute;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_address32_attribute {
    pub(crate) Granularity: u32,
    pub(crate) Minimum: u32,
    pub(crate) Maximum: u32,
    pub(crate) TranslationOffset: u32,
    pub(crate) AddressLength: u32,
}
pub(crate) type ACPI_ADDRESS32_ATTRIBUTE = acpi_address32_attribute;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_address64_attribute {
    pub(crate) Granularity: u64,
    pub(crate) Minimum: u64,
    pub(crate) Maximum: u64,
    pub(crate) TranslationOffset: u64,
    pub(crate) AddressLength: u64,
}
pub(crate) type ACPI_ADDRESS64_ATTRIBUTE = acpi_address64_attribute;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_resource_address {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: ACPI_RESOURCE_ATTRIBUTE,
}
pub(crate) type ACPI_RESOURCE_ADDRESS = acpi_resource_address;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_resource_address16 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: ACPI_RESOURCE_ATTRIBUTE,
    pub(crate) Address: ACPI_ADDRESS16_ATTRIBUTE,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
}
pub(crate) type ACPI_RESOURCE_ADDRESS16 = acpi_resource_address16;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_resource_address32 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: ACPI_RESOURCE_ATTRIBUTE,
    pub(crate) Address: ACPI_ADDRESS32_ATTRIBUTE,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
}
pub(crate) type ACPI_RESOURCE_ADDRESS32 = acpi_resource_address32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_resource_address64 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: ACPI_RESOURCE_ATTRIBUTE,
    pub(crate) Address: ACPI_ADDRESS64_ATTRIBUTE,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
}
pub(crate) type ACPI_RESOURCE_ADDRESS64 = acpi_resource_address64;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_resource_extended_address64 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: ACPI_RESOURCE_ATTRIBUTE,
    pub(crate) RevisionID: u8,
    pub(crate) Address: ACPI_ADDRESS64_ATTRIBUTE,
    pub(crate) TypeSpecific: u64,
}
pub(crate) type ACPI_RESOURCE_EXTENDED_ADDRESS64 = acpi_resource_extended_address64;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_extended_irq {
    pub(crate) ProducerConsumer: u8,
    pub(crate) Triggering: u8,
    pub(crate) Polarity: u8,
    pub(crate) Shareable: u8,
    pub(crate) WakeCapable: u8,
    pub(crate) InterruptCount: u8,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) Interrupts: [u32; 1usize],
}
pub(crate) type ACPI_RESOURCE_EXTENDED_IRQ = acpi_resource_extended_irq;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_generic_register {
    pub(crate) SpaceId: u8,
    pub(crate) BitWidth: u8,
    pub(crate) BitOffset: u8,
    pub(crate) AccessSize: u8,
    pub(crate) Address: u64,
}
pub(crate) type ACPI_RESOURCE_GENERIC_REGISTER = acpi_resource_generic_register;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_gpio {
    pub(crate) RevisionId: u8,
    pub(crate) ConnectionType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) PinConfig: u8,
    pub(crate) Shareable: u8,
    pub(crate) WakeCapable: u8,
    pub(crate) IoRestriction: u8,
    pub(crate) Triggering: u8,
    pub(crate) Polarity: u8,
    pub(crate) DriveStrength: u16,
    pub(crate) DebounceTimeout: u16,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) PinTable: *mut u16,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_GPIO = acpi_resource_gpio;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_common_serialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_COMMON_SERIALBUS = acpi_resource_common_serialbus;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_i2c_serialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) VendorData: *mut u8,
    pub(crate) AccessMode: u8,
    pub(crate) SlaveAddress: u16,
    pub(crate) ConnectionSpeed: u32,
}
pub(crate) type ACPI_RESOURCE_I2C_SERIALBUS = acpi_resource_i2c_serialbus;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_spi_serialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) VendorData: *mut u8,
    pub(crate) WireMode: u8,
    pub(crate) DevicePolarity: u8,
    pub(crate) DataBitLength: u8,
    pub(crate) ClockPhase: u8,
    pub(crate) ClockPolarity: u8,
    pub(crate) DeviceSelection: u16,
    pub(crate) ConnectionSpeed: u32,
}
pub(crate) type ACPI_RESOURCE_SPI_SERIALBUS = acpi_resource_spi_serialbus;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_uart_serialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) VendorData: *mut u8,
    pub(crate) Endian: u8,
    pub(crate) DataBits: u8,
    pub(crate) StopBits: u8,
    pub(crate) FlowControl: u8,
    pub(crate) Parity: u8,
    pub(crate) LinesEnabled: u8,
    pub(crate) RxFifoSize: u16,
    pub(crate) TxFifoSize: u16,
    pub(crate) DefaultBaudRate: u32,
}
pub(crate) type ACPI_RESOURCE_UART_SERIALBUS = acpi_resource_uart_serialbus;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_csi2_serialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) VendorData: *mut u8,
    pub(crate) LocalPortInstance: u8,
    pub(crate) PhyType: u8,
}
pub(crate) type ACPI_RESOURCE_CSI2_SERIALBUS = acpi_resource_csi2_serialbus;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_pin_function {
    pub(crate) RevisionId: u8,
    pub(crate) PinConfig: u8,
    pub(crate) Shareable: u8,
    pub(crate) FunctionNumber: u16,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) PinTable: *mut u16,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_PIN_FUNCTION = acpi_resource_pin_function;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_pin_config {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Shareable: u8,
    pub(crate) PinConfigType: u8,
    pub(crate) PinConfigValue: u32,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) PinTable: *mut u16,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_PIN_CONFIG = acpi_resource_pin_config;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_pin_group {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) PinTable: *mut u16,
    pub(crate) ResourceLabel: ACPI_RESOURCE_LABEL,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_PIN_GROUP = acpi_resource_pin_group;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_pin_group_function {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Shareable: u8,
    pub(crate) FunctionNumber: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) ResourceSourceLabel: ACPI_RESOURCE_LABEL,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_PIN_GROUP_FUNCTION = acpi_resource_pin_group_function;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_resource_pin_group_config {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Shareable: u8,
    pub(crate) PinConfigType: u8,
    pub(crate) PinConfigValue: u32,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: ACPI_RESOURCE_SOURCE,
    pub(crate) ResourceSourceLabel: ACPI_RESOURCE_LABEL,
    pub(crate) VendorData: *mut u8,
}
pub(crate) type ACPI_RESOURCE_PIN_GROUP_CONFIG = acpi_resource_pin_group_config;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union acpi_resource_data {
    pub(crate) Irq: ACPI_RESOURCE_IRQ,
    pub(crate) Dma: ACPI_RESOURCE_DMA,
    pub(crate) StartDpf: ACPI_RESOURCE_START_DEPENDENT,
    pub(crate) Io: ACPI_RESOURCE_IO,
    pub(crate) FixedIo: ACPI_RESOURCE_FIXED_IO,
    pub(crate) FixedDma: ACPI_RESOURCE_FIXED_DMA,
    pub(crate) Vendor: ACPI_RESOURCE_VENDOR,
    pub(crate) VendorTyped: ACPI_RESOURCE_VENDOR_TYPED,
    pub(crate) EndTag: ACPI_RESOURCE_END_TAG,
    pub(crate) Memory24: ACPI_RESOURCE_MEMORY24,
    pub(crate) Memory32: ACPI_RESOURCE_MEMORY32,
    pub(crate) FixedMemory32: ACPI_RESOURCE_FIXED_MEMORY32,
    pub(crate) Address16: ACPI_RESOURCE_ADDRESS16,
    pub(crate) Address32: ACPI_RESOURCE_ADDRESS32,
    pub(crate) Address64: ACPI_RESOURCE_ADDRESS64,
    pub(crate) ExtAddress64: ACPI_RESOURCE_EXTENDED_ADDRESS64,
    pub(crate) ExtendedIrq: ACPI_RESOURCE_EXTENDED_IRQ,
    pub(crate) GenericReg: ACPI_RESOURCE_GENERIC_REGISTER,
    pub(crate) Gpio: ACPI_RESOURCE_GPIO,
    pub(crate) I2cSerialBus: ACPI_RESOURCE_I2C_SERIALBUS,
    pub(crate) SpiSerialBus: ACPI_RESOURCE_SPI_SERIALBUS,
    pub(crate) UartSerialBus: ACPI_RESOURCE_UART_SERIALBUS,
    pub(crate) Csi2SerialBus: ACPI_RESOURCE_CSI2_SERIALBUS,
    pub(crate) CommonSerialBus: ACPI_RESOURCE_COMMON_SERIALBUS,
    pub(crate) PinFunction: ACPI_RESOURCE_PIN_FUNCTION,
    pub(crate) PinConfig: ACPI_RESOURCE_PIN_CONFIG,
    pub(crate) PinGroup: ACPI_RESOURCE_PIN_GROUP,
    pub(crate) PinGroupFunction: ACPI_RESOURCE_PIN_GROUP_FUNCTION,
    pub(crate) PinGroupConfig: ACPI_RESOURCE_PIN_GROUP_CONFIG,
    pub(crate) Address: ACPI_RESOURCE_ADDRESS,
}
pub(crate) type ACPI_RESOURCE_DATA = acpi_resource_data;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct acpi_resource {
    pub(crate) Type: u32,
    pub(crate) Length: u32,
    pub(crate) Data: ACPI_RESOURCE_DATA,
}
pub(crate) type ACPI_RESOURCE = acpi_resource;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_pci_routing_table {
    pub(crate) Length: u32,
    pub(crate) Pin: u32,
    pub(crate) Address: u64,
    pub(crate) SourceIndex: u32,
    pub(crate) Source: [i8; 4usize],
}
pub(crate) type ACPI_PCI_ROUTING_TABLE = acpi_pci_routing_table;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum ACPI_EXECUTE_TYPE {
    OSL_GLOBAL_LOCK_HANDLER = 0,
    OSL_NOTIFY_HANDLER = 1,
    OSL_GPE_HANDLER = 2,
    OSL_DEBUGGER_MAIN_THREAD = 3,
    OSL_DEBUGGER_EXEC_THREAD = 4,
    OSL_EC_POLL_HANDLER = 5,
    OSL_EC_BURST_HANDLER = 6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_signal_fatal_info {
    pub(crate) Type: u32,
    pub(crate) Code: u32,
    pub(crate) Argument: u32,
}
pub(crate) type ACPI_SIGNAL_FATAL_INFO = acpi_signal_fatal_info;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_fde_info {
    pub(crate) Floppy0: u32,
    pub(crate) Floppy1: u32,
    pub(crate) Floppy2: u32,
    pub(crate) Floppy3: u32,
    pub(crate) Tape: u32,
}
pub(crate) type ACPI_FDE_INFO = acpi_fde_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_grt_info {
    pub(crate) Year: u16,
    pub(crate) Month: u8,
    pub(crate) Day: u8,
    pub(crate) Hour: u8,
    pub(crate) Minute: u8,
    pub(crate) Second: u8,
    pub(crate) Valid: u8,
    pub(crate) Milliseconds: u16,
    pub(crate) Timezone: u16,
    pub(crate) Daylight: u8,
    pub(crate) Reserved: [u8; 3usize],
}
pub(crate) type ACPI_GRT_INFO = acpi_grt_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_gtm_info {
    pub(crate) PioSpeed0: u32,
    pub(crate) DmaSpeed0: u32,
    pub(crate) PioSpeed1: u32,
    pub(crate) DmaSpeed1: u32,
    pub(crate) Flags: u32,
}
pub(crate) type ACPI_GTM_INFO = acpi_gtm_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct acpi_pld_info {
    pub(crate) Revision: u8,
    pub(crate) IgnoreColor: u8,
    pub(crate) Red: u8,
    pub(crate) Green: u8,
    pub(crate) Blue: u8,
    pub(crate) Width: u16,
    pub(crate) Height: u16,
    pub(crate) UserVisible: u8,
    pub(crate) Dock: u8,
    pub(crate) Lid: u8,
    pub(crate) Panel: u8,
    pub(crate) VerticalPosition: u8,
    pub(crate) HorizontalPosition: u8,
    pub(crate) Shape: u8,
    pub(crate) GroupOrientation: u8,
    pub(crate) GroupToken: u8,
    pub(crate) GroupPosition: u8,
    pub(crate) Bay: u8,
    pub(crate) Ejectable: u8,
    pub(crate) OspmEjectRequired: u8,
    pub(crate) CabinetNumber: u8,
    pub(crate) CardCageNumber: u8,
    pub(crate) Reference: u8,
    pub(crate) Rotation: u8,
    pub(crate) Order: u8,
    pub(crate) Reserved: u8,
    pub(crate) VerticalOffset: u16,
    pub(crate) HorizontalOffset: u16,
}

pub(crate) type ACPI_PLD_INFO = acpi_pld_info;
