pub(crate) mod tables;
pub mod functions;
pub mod object;

// pub(crate) use tables::*;
use tables::FfiAcpiTableHeader;

use crate::interface::status::AcpiStatus;

use self::object::{FfiAcpiObject, FfiAcpiObjectType};

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
pub(crate) type FfiAcpiSize = u64;
pub(crate) type FfiAcpiIoAddress = u64;
pub(crate) type FfiAcpiPhysicalAddress = usize;
pub(crate) type FfiAcpiName = u32;
pub(crate) type FfiAcpiString = *mut i8;
pub(crate) type FfiAcpiHandle = *mut ::core::ffi::c_void;
pub(crate) type FfiAcpiOwnerId = u16;
pub(crate) type FfiAcpiEventStatus = u32;
pub(crate) type FfiAcpiAdtSpaceType = u8;

///  GAS - Generic Address Structure (ACPI 2.0+)
/// 
///  Note: Since this structure is used in the ACPI tables, it is byte aligned.
///  If misaligned access is not supported by the hardware, accesses to the
///  64-bit Address field must be performed with care.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiGenericAddress {
    pub(crate) space_id: u8,
    pub(crate) bit_width: u8,
    pub(crate) bit_offset: u8,
    pub(crate) access_width: u8,
    pub(crate) address: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiObjectList {
    pub(crate) count: u32,
    pub(crate) pointer: *mut FfiAcpiObject,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiBuffer {
    pub(crate) length: FfiAcpiSize,
    pub(crate) pointer: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiPredefinedNames {
    pub(crate) name: *const i8,
    pub(crate) object_type: u8,
    pub(crate) val: *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiStatistics {
    pub(crate) sci_count: u32,
    pub(crate) gpe_count: u32,
    pub(crate) FixedEventCount: [u32; 5usize],
    pub(crate) method_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiPnpDeviceId {
    pub(crate) length: u32,
    pub(crate) string: *mut i8,
}
#[repr(C)]
#[derive(Debug)]
pub(crate) struct FfiAcpiPnpDeviceIdList {
    pub(crate) count: u32,
    pub(crate) list_size: u32,
    pub(crate) ids: __IncompleteArrayField<FfiAcpiPnpDeviceId>,
}
#[repr(C)]
#[derive(Debug)]
pub(crate) struct FfiAcpiDeviceInfo {
    pub(crate) info_size: u32,
    pub(crate) name: u32,
    pub(crate) object_type: FfiAcpiObjectType,
    pub(crate) param_count: u8,
    pub(crate) valid: u16,
    pub(crate) flags: u8,
    pub(crate) highest_dstates: [u8; 4usize],
    pub(crate) lowest_dstates: [u8; 5usize],
    pub(crate) address: u64,
    pub(crate) hardware_id: FfiAcpiPnpDeviceId,
    pub(crate) unique_id: FfiAcpiPnpDeviceId,
    pub(crate) class_code: FfiAcpiPnpDeviceId,
    pub(crate) compatible_id_list: FfiAcpiPnpDeviceIdList,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiPciId {
    pub(crate) Segment: u16,
    pub(crate) Bus: u16,
    pub(crate) Device: u16,
    pub(crate) Function: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiMemMapping {
    pub(crate) physical_address: FfiAcpiPhysicalAddress,
    pub(crate) logical_address: *mut u8,
    pub(crate) length: FfiAcpiSize,
    pub(crate) next_mm: *mut FfiAcpiMemMapping,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiMemSpaceContext {
    pub(crate) length: u32,
    pub(crate) address: FfiAcpiPhysicalAddress,
    pub(crate) cur_mm: *mut FfiAcpiMemMapping,
    pub(crate) first_mm: *mut FfiAcpiMemMapping,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiMemoryList {
    pub(crate) ListName: *const i8,
    pub(crate) ListHead: *mut ::core::ffi::c_void,
    pub(crate) ObjectSize: u16,
    pub(crate) MaxDepth: u16,
    pub(crate) CurrentDepth: u16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(dead_code)] // FFI type so variants are not explicitly constructed
pub(crate) enum FfiAcpiTraceEventType {
    Method = 0,
    Opcode = 1,
    Region = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiExceptionInfo {
    pub(crate) name: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FfiAcpiNameUnion {
    pub(crate) Integer: u32,
    pub(crate) Ascii: [i8; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiTableDesc {
    pub(crate) Address: FfiAcpiPhysicalAddress,
    pub(crate) Pointer: *mut FfiAcpiTableHeader,
    pub(crate) Length: u32,
    pub(crate) Signature: FfiAcpiNameUnion,
    pub(crate) OwnerId: FfiAcpiOwnerId,
    pub(crate) Flags: u8,
    pub(crate) ValidationCount: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiWheaHeader {
    pub(crate) Action: u8,
    pub(crate) Instruction: u8,
    pub(crate) Flags: u8,
    pub(crate) Reserved: u8,
    pub(crate) RegisterRegion: FfiAcpiGenericAddress,
    pub(crate) Value: u64,
    pub(crate) Mask: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiUuid {
    pub(crate) Data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiVendorUuid {
    pub(crate) Subtype: u8,
    pub(crate) Data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceIrq {
    pub(crate) DescriptorLength: u8,
    pub(crate) Triggering: u8,
    pub(crate) Polarity: u8,
    pub(crate) Shareable: u8,
    pub(crate) WakeCapable: u8,
    pub(crate) InterruptCount: u8,
    pub(crate) Interrupts: [u8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceDma {
    pub(crate) Type: u8,
    pub(crate) BusMaster: u8,
    pub(crate) Transfer: u8,
    pub(crate) ChannelCount: u8,
    pub(crate) Channels: [u8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceStartDependent {
    pub(crate) DescriptorLength: u8,
    pub(crate) CompatibilityPriority: u8,
    pub(crate) PerformanceRobustness: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceIo {
    pub(crate) IoDecode: u8,
    pub(crate) Alignment: u8,
    pub(crate) AddressLength: u8,
    pub(crate) Minimum: u16,
    pub(crate) Maximum: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceFixedIo {
    pub(crate) Address: u16,
    pub(crate) AddressLength: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceFixedDma {
    pub(crate) RequestLines: u16,
    pub(crate) Channels: u16,
    pub(crate) Width: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceVendor {
    pub(crate) ByteLength: u16,
    pub(crate) ByteData: [u8; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceVendorTyped {
    pub(crate) ByteLength: u16,
    pub(crate) UuidSubtype: u8,
    pub(crate) Uuid: [u8; 16usize],
    pub(crate) ByteData: [u8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceEndTag {
    pub(crate) Checksum: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceMemory24 {
    pub(crate) WriteProtect: u8,
    pub(crate) Minimum: u16,
    pub(crate) Maximum: u16,
    pub(crate) Alignment: u16,
    pub(crate) AddressLength: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceMemory32 {
    pub(crate) WriteProtect: u8,
    pub(crate) Minimum: u32,
    pub(crate) Maximum: u32,
    pub(crate) Alignment: u32,
    pub(crate) AddressLength: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceFixedMemory32 {
    pub(crate) WriteProtect: u8,
    pub(crate) Address: u32,
    pub(crate) AddressLength: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiMemoryAttribute {
    pub(crate) WriteProtect: u8,
    pub(crate) Caching: u8,
    pub(crate) RangeType: u8,
    pub(crate) Translation: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiIoAttribute {
    pub(crate) RangeType: u8,
    pub(crate) Translation: u8,
    pub(crate) TranslationType: u8,
    pub(crate) Reserved1: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FfiAcpiResourceAttribute {
    pub(crate) Mem: FfiAcpiMemoryAttribute,
    pub(crate) Io: FfiAcpiIoAttribute,
    pub(crate) TypeSpecific: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceLabel {
    pub(crate) StringLength: u16,
    pub(crate) StringPtr: *mut i8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceSource {
    pub(crate) Index: u8,
    pub(crate) StringLength: u16,
    pub(crate) StringPtr: *mut i8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiAddress16Attribute {
    pub(crate) Granularity: u16,
    pub(crate) Minimum: u16,
    pub(crate) Maximum: u16,
    pub(crate) TranslationOffset: u16,
    pub(crate) AddressLength: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiAddress32Attribute {
    pub(crate) Granularity: u32,
    pub(crate) Minimum: u32,
    pub(crate) Maximum: u32,
    pub(crate) TranslationOffset: u32,
    pub(crate) AddressLength: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiAddress64Attribute {
    pub(crate) Granularity: u64,
    pub(crate) Minimum: u64,
    pub(crate) Maximum: u64,
    pub(crate) TranslationOffset: u64,
    pub(crate) AddressLength: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiResourceAddress {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: FfiAcpiResourceAttribute,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiResourceAddress16 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: FfiAcpiResourceAttribute,
    pub(crate) Address: FfiAcpiAddress16Attribute,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiResourceAddress32 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: FfiAcpiResourceAttribute,
    pub(crate) Address: FfiAcpiAddress32Attribute,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiResourceAddress64 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: FfiAcpiResourceAttribute,
    pub(crate) Address: FfiAcpiAddress64Attribute,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiResourceExtendedAddress64 {
    pub(crate) ResourceType: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Decode: u8,
    pub(crate) MinAddressFixed: u8,
    pub(crate) MaxAddressFixed: u8,
    pub(crate) Info: FfiAcpiResourceAttribute,
    pub(crate) RevisionID: u8,
    pub(crate) Address: FfiAcpiAddress64Attribute,
    pub(crate) TypeSpecific: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceExtendedIrq {
    pub(crate) ProducerConsumer: u8,
    pub(crate) Triggering: u8,
    pub(crate) Polarity: u8,
    pub(crate) Shareable: u8,
    pub(crate) WakeCapable: u8,
    pub(crate) InterruptCount: u8,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) Interrupts: [u32; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceGenericRegister {
    pub(crate) SpaceId: u8,
    pub(crate) BitWidth: u8,
    pub(crate) BitOffset: u8,
    pub(crate) AccessSize: u8,
    pub(crate) Address: u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceGpio {
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
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) PinTable: *mut u16,
    pub(crate) VendorData: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceCommonSerialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) VendorData: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceI2cSerialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) VendorData: *mut u8,
    pub(crate) AccessMode: u8,
    pub(crate) SlaveAddress: u16,
    pub(crate) ConnectionSpeed: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceSpiSerialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) VendorData: *mut u8,
    pub(crate) WireMode: u8,
    pub(crate) DevicePolarity: u8,
    pub(crate) DataBitLength: u8,
    pub(crate) ClockPhase: u8,
    pub(crate) ClockPolarity: u8,
    pub(crate) DeviceSelection: u16,
    pub(crate) ConnectionSpeed: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceUartSerialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourceCsi2Serialbus {
    pub(crate) RevisionId: u8,
    pub(crate) Type: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) SlaveMode: u8,
    pub(crate) ConnectionSharing: u8,
    pub(crate) TypeRevisionId: u8,
    pub(crate) TypeDataLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) VendorData: *mut u8,
    pub(crate) LocalPortInstance: u8,
    pub(crate) PhyType: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourcePinFunction {
    pub(crate) RevisionId: u8,
    pub(crate) PinConfig: u8,
    pub(crate) Shareable: u8,
    pub(crate) FunctionNumber: u16,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) PinTable: *mut u16,
    pub(crate) VendorData: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourcePinConfig {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Shareable: u8,
    pub(crate) PinConfigType: u8,
    pub(crate) PinConfigValue: u32,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) PinTable: *mut u16,
    pub(crate) VendorData: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourcePinGroup {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) PinTableLength: u16,
    pub(crate) VendorLength: u16,
    pub(crate) PinTable: *mut u16,
    pub(crate) ResourceLabel: FfiAcpiResourceLabel,
    pub(crate) VendorData: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourcePinGroupFunction {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Shareable: u8,
    pub(crate) FunctionNumber: u16,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) ResourceSourceLabel: FfiAcpiResourceLabel,
    pub(crate) VendorData: *mut u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiResourcePinGroupConfig {
    pub(crate) RevisionId: u8,
    pub(crate) ProducerConsumer: u8,
    pub(crate) Shareable: u8,
    pub(crate) PinConfigType: u8,
    pub(crate) PinConfigValue: u32,
    pub(crate) VendorLength: u16,
    pub(crate) ResourceSource: FfiAcpiResourceSource,
    pub(crate) ResourceSourceLabel: FfiAcpiResourceLabel,
    pub(crate) VendorData: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FfiAcpiResourceData {
    pub(crate) Irq: FfiAcpiResourceIrq,
    pub(crate) Dma: FfiAcpiResourceDma,
    pub(crate) StartDpf: FfiAcpiResourceStartDependent,
    pub(crate) Io: FfiAcpiResourceIo,
    pub(crate) FixedIo: FfiAcpiResourceFixedIo,
    pub(crate) FixedDma: FfiAcpiResourceFixedDma,
    pub(crate) Vendor: FfiAcpiResourceVendor,
    pub(crate) VendorTyped: FfiAcpiResourceVendorTyped,
    pub(crate) EndTag: FfiAcpiResourceEndTag,
    pub(crate) Memory24: FfiAcpiResourceMemory24,
    pub(crate) Memory32: FfiAcpiResourceMemory32,
    pub(crate) FixedMemory32: FfiAcpiResourceFixedMemory32,
    pub(crate) Address16: FfiAcpiResourceAddress16,
    pub(crate) Address32: FfiAcpiResourceAddress32,
    pub(crate) Address64: FfiAcpiResourceAddress64,
    pub(crate) ExtAddress64: FfiAcpiResourceExtendedAddress64,
    pub(crate) ExtendedIrq: FfiAcpiResourceExtendedIrq,
    pub(crate) GenericReg: FfiAcpiResourceGenericRegister,
    pub(crate) Gpio: FfiAcpiResourceGpio,
    pub(crate) I2cSerialBus: FfiAcpiResourceI2cSerialbus,
    pub(crate) SpiSerialBus: FfiAcpiResourceSpiSerialbus,
    pub(crate) UartSerialBus: FfiAcpiResourceUartSerialbus,
    pub(crate) Csi2SerialBus: FfiAcpiResourceCsi2Serialbus,
    pub(crate) CommonSerialBus: FfiAcpiResourceCommonSerialbus,
    pub(crate) PinFunction: FfiAcpiResourcePinFunction,
    pub(crate) PinConfig: FfiAcpiResourcePinConfig,
    pub(crate) PinGroup: FfiAcpiResourcePinGroup,
    pub(crate) PinGroupFunction: FfiAcpiResourcePinGroupFunction,
    pub(crate) PinGroupConfig: FfiAcpiResourcePinGroupConfig,
    pub(crate) Address: FfiAcpiResourceAddress,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct FfiAcpiResource {
    pub(crate) Type: u32,
    pub(crate) Length: u32,
    pub(crate) Data: FfiAcpiResourceData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiPciRoutingTable {
    pub(crate) Length: u32,
    pub(crate) Pin: u32,
    pub(crate) Address: u64,
    pub(crate) SourceIndex: u32,
    pub(crate) Source: [i8; 4usize],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(dead_code)] // FFI type so variants are not explicitly constructed
pub(crate) enum FfiAcpiExecuteType {
    GlobalLockHandler = 0,
    NotifyHandler = 1,
    GpeHandler = 2,
    DebuggerMainThread = 3,
    DebuggerExecThread = 4,
    EcPollHandler = 5,
    EcBurstHandler = 6,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiSignalFatalInfo {
    pub(crate) Type: u32,
    pub(crate) Code: u32,
    pub(crate) Argument: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiFdeInfo {
    pub(crate) Floppy0: u32,
    pub(crate) Floppy1: u32,
    pub(crate) Floppy2: u32,
    pub(crate) Floppy3: u32,
    pub(crate) Tape: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiGrtInfo {
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiGtmInfo {
    pub(crate) PioSpeed0: u32,
    pub(crate) DmaSpeed0: u32,
    pub(crate) PioSpeed1: u32,
    pub(crate) DmaSpeed1: u32,
    pub(crate) Flags: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct FfiAcpiPldInfo {
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