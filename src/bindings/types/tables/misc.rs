use crate::{bindings::types::FfiAcpiTableHeader, bindings::types::__IncompleteArrayField, interface::AcpiGenericAddress};

use super::FfiAcpiSubtableHeader;

///  ASF - Alert Standard Format table (Signature \"ASF!\")
///        Revision 0x10
/// 
///  Conforms to the Alert Standard Format Specification V2.0, 23 April 2003
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableAsf {
    pub Header: FfiAcpiTableHeader,
}
///  ASF - Alert Standard Format table (Signature \"ASF!\")
///        Revision 0x10
/// 
///  Conforms to the Alert Standard Format Specification V2.0, 23 April 2003
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfHeader {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiAsfType {
    ACPI_ASF_TYPE_INFO = 0,
    ACPI_ASF_TYPE_ALERT = 1,
    ACPI_ASF_TYPE_CONTROL = 2,
    ACPI_ASF_TYPE_BOOT = 3,
    ACPI_ASF_TYPE_ADDRESS = 4,
    ACPI_ASF_TYPE_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfInfo {
    pub Header: FfiAcpiAsfHeader,
    pub MinResetValue: u8,
    pub MinPollInterval: u8,
    pub SystemId: u16,
    pub MfgId: u32,
    pub Flags: u8,
    pub Reserved2: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfAlert {
    pub Header: FfiAcpiAsfHeader,
    pub AssertMask: u8,
    pub DeassertMask: u8,
    pub Alerts: u8,
    pub DataLength: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfAlertData {
    pub Address: u8,
    pub Command: u8,
    pub Mask: u8,
    pub Value: u8,
    pub SensorType: u8,
    pub Type: u8,
    pub Offset: u8,
    pub SourceType: u8,
    pub Severity: u8,
    pub SensorNumber: u8,
    pub Entity: u8,
    pub Instance: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfRemote {
    pub Header: FfiAcpiAsfHeader,
    pub Controls: u8,
    pub DataLength: u8,
    pub Reserved2: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfControlData {
    pub Function: u8,
    pub Address: u8,
    pub Command: u8,
    pub Value: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfRmcp {
    pub Header: FfiAcpiAsfHeader,
    pub Capabilities: [u8; 7usize],
    pub CompletionCode: u8,
    pub EnterpriseId: u32,
    pub Command: u8,
    pub Parameter: u16,
    pub BootOptions: u16,
    pub OemParameters: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiAsfAddress {
    pub Header: FfiAcpiAsfHeader,
    pub EpromAddress: u8,
    pub Devices: u8,
}

///  BERT - Boot Error Record Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableBert {
    pub Header: FfiAcpiTableHeader,
    pub RegionLength: u32,
    pub Address: u64,
}
///  BERT - Boot Error Record Table (ACPI 4.0)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiBertRegion {
    pub BlockStatus: u32,
    pub RawDataOffset: u32,
    pub RawDataLength: u32,
    pub DataLength: u32,
    pub ErrorSeverity: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiBertErrorSeverity {
    ACPI_BERT_ERROR_CORRECTABLE = 0,
    ACPI_BERT_ERROR_FATAL = 1,
    ACPI_BERT_ERROR_CORRECTED = 2,
    ACPI_BERT_ERROR_NONE = 3,
    ACPI_BERT_ERROR_RESERVED = 4,
}
///  BGRT - Boot Graphics Resource Table (ACPI 5.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableBgrt {
    pub Header: FfiAcpiTableHeader,
    pub Version: u16,
    pub Status: u8,
    pub ImageType: u8,
    pub ImageAddress: u64,
    pub ImageOffsetX: u32,
    pub ImageOffsetY: u32,
}
///  BGRT - Boot Graphics Resource Table (ACPI 5.0)
///         Version 1
/// 

///  BOOT - Simple Boot Flag Table
///         Version 1
/// 
///  Conforms to the \"Simple Boot Flag Specification\", Version 2.1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableBoot {
    pub Header: FfiAcpiTableHeader,
    pub CmosIndex: u8,
    pub Reserved: [u8; 3usize],
}
///  BOOT - Simple Boot Flag Table
///         Version 1
/// 
///  Conforms to the \"Simple Boot Flag Specification\", Version 2.1
/// 


///  CPEP - Corrected Platform Error Polling table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableCpep {
    pub Header: FfiAcpiTableHeader,
    pub Reserved: u64,
}
///  CPEP - Corrected Platform Error Polling table (ACPI 4.0)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiCpepPolling {
    pub Header: FfiAcpiSubtableHeader,
    pub Id: u8,
    pub Eid: u8,
    pub Interval: u32,
}



///  DBGP - Debug Port table
///         Version 1
/// 
///  Conforms to the \"Debug Port Specification\", Version 1.00, 2/9/2000
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableDbgp {
    pub Header: FfiAcpiTableHeader,
    pub Type: u8,
    pub Reserved: [u8; 3usize],
    pub DebugPort: AcpiGenericAddress,
}
///  DBGP - Debug Port table
///         Version 1
/// 
///  Conforms to the \"Debug Port Specification\", Version 1.00, 2/9/2000
/// 



///  ECDT - Embedded Controller Boot Resources Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableEcdt {
    pub Header: FfiAcpiTableHeader,
    pub Control: AcpiGenericAddress,
    pub Data: AcpiGenericAddress,
    pub Uid: u32,
    pub Gpe: u8,
    pub Id: [u8; 1usize],
}
///  ECDT - Embedded Controller Boot Resources Table
///         Version 1
/// 



///  BDAT - BIOS Data ACPI Table
/// 
///  Conforms to \"BIOS Data ACPI Table\", Interface Specification v4.0 Draft 5
///  Nov 2020
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableBdat {
    pub Header: FfiAcpiTableHeader,
    pub Gas: AcpiGenericAddress,
}
///  BDAT - BIOS Data ACPI Table
/// 
///  Conforms to \"BIOS Data ACPI Table\", Interface Specification v4.0 Draft 5
///  Nov 2020
/// 




///  LPIT - Low Power Idle Table
/// 
///  Conforms to \"ACPI Low Power Idle Table (LPIT)\" July 2014.
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableLpit {
    pub Header: FfiAcpiTableHeader,
}
///  LPIT - Low Power Idle Table
/// 
///  Conforms to \"ACPI Low Power Idle Table (LPIT)\" July 2014.
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiLpitHeader {
    pub Type: u32,
    pub Length: u32,
    pub UniqueId: u16,
    pub Reserved: u16,
    pub Flags: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiLpitType {
    ACPI_LPIT_TYPE_NATIVE_CSTATE = 0,
    ACPI_LPIT_TYPE_RESERVED = 1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiLpitNative {
    pub Header: FfiAcpiLpitHeader,
    pub EntryTrigger: AcpiGenericAddress,
    pub Residency: u32,
    pub Latency: u32,
    pub ResidencyCounter: AcpiGenericAddress,
    pub CounterFrequency: u64,
}


///  MCFG - PCI Memory Mapped Configuration table and subtable
///         Version 1
/// 
///  Conforms to \"PCI Firmware Specification\", Revision 3.0, June 20, 2005
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableMcfg {
    pub Header: FfiAcpiTableHeader,
    pub Reserved: [u8; 8usize],
}
///  MCFG - PCI Memory Mapped Configuration table and subtable
///         Version 1
/// 
///  Conforms to \"PCI Firmware Specification\", Revision 3.0, June 20, 2005
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMcfgAllocation {
    pub Address: u64,
    pub PciSegment: u16,
    pub StartBusNumber: u8,
    pub EndBusNumber: u8,
    pub Reserved: u32,
}

///  MCHI - Management Controller Host Interface Table
///         Version 1
/// 
///  Conforms to \"Management Component Transport Protocol (MCTP) Host
///  Interface Specification\", Revision 1.0.0a, October 13, 2009
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableMchi {
    pub Header: FfiAcpiTableHeader,
    pub InterfaceType: u8,
    pub Protocol: u8,
    pub ProtocolData: u64,
    pub InterruptType: u8,
    pub Gpe: u8,
    pub PciDeviceFlag: u8,
    pub GlobalInterrupt: u32,
    pub ControlRegister: AcpiGenericAddress,
    pub PciSegment: u8,
    pub PciBus: u8,
    pub PciDevice: u8,
    pub PciFunction: u8,
}
///  MCHI - Management Controller Host Interface Table
///         Version 1
/// 
///  Conforms to \"Management Component Transport Protocol (MCTP) Host
///  Interface Specification\", Revision 1.0.0a, October 13, 2009
/// 


///  MSCT - Maximum System Characteristics Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableMsct {
    pub Header: FfiAcpiTableHeader,
    pub ProximityOffset: u32,
    pub MaxProximityDomains: u32,
    pub MaxClockDomains: u32,
    pub MaxAddress: u64,
}
///  MSCT - Maximum System Characteristics Table (ACPI 4.0)
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiMsctProximity {
    pub Revision: u8,
    pub Length: u8,
    pub RangeStart: u32,
    pub RangeEnd: u32,
    pub ProcessorCapacity: u32,
    pub MemoryCapacity: u64,
}

///  MSDM - Microsoft Data Management table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableMsdm {
    pub Header: FfiAcpiTableHeader,
}
///  MSDM - Microsoft Data Management table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 




///  PDTT - Platform Debug Trigger Table (ACPI 6.2)
///         Version 0
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTablePdtt {
    pub Header: FfiAcpiTableHeader,
    pub TriggerCount: u8,
    pub Reserved: [u8; 3usize],
    pub ArrayOffset: u32,
}
///  PDTT - Platform Debug Trigger Table (ACPI 6.2)
///         Version 0
/// 

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiPdttChannel {
    pub SubchannelId: u8,
    pub Flags: u8,
}



///  RGRT - Regulatory Graphics Resource Table
///         Version 1
/// 
///  Conforms to \"ACPI RGRT\" available at:
///  https://microsoft.github.io/mu/dyn/mu_plus/MsCorePkg/AcpiRGRT/feature_acpi_rgrt/
/// 
#[repr(C, packed)]
pub struct FfiAcpiTableRgrt {
    pub Header: FfiAcpiTableHeader,
    pub Version: u16,
    pub ImageType: u8,
    pub Reserved: u8,
    Image: __IncompleteArrayField<u8>,
}
///  RGRT - Regulatory Graphics Resource Table
///         Version 1
/// 
///  Conforms to \"ACPI RGRT\" available at:
///  https://microsoft.github.io/mu/dyn/mu_plus/MsCorePkg/AcpiRGRT/feature_acpi_rgrt/
/// 

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiRgrtImageType {
    ACPI_RGRT_TYPE_RESERVED0 = 0,
    ACPI_RGRT_IMAGE_TYPE_PNG = 1,
    ACPI_RGRT_TYPE_RESERVED = 2,
}
///  SBST - Smart Battery Specification Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSbst {
    pub Header: FfiAcpiTableHeader,
    pub WarningLevel: u32,
    pub LowLevel: u32,
    pub CriticalLevel: u32,
}
///  SBST - Smart Battery Specification Table
///         Version 1
/// 

///  SDEI - Software Delegated Exception Interface Descriptor Table
/// 
///  Conforms to \"Software Delegated Exception Interface (SDEI)\" ARM DEN0054A,
///  May 8th, 2017. Copyright 2017 ARM Ltd.
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSdei {
    pub Header: FfiAcpiTableHeader,
}
///  SDEI - Software Delegated Exception Interface Descriptor Table
/// 
///  Conforms to \"Software Delegated Exception Interface (SDEI)\" ARM DEN0054A,
///  May 8th, 2017. Copyright 2017 ARM Ltd.
/// 



///  SVKL - Storage Volume Key Location Table (ACPI 6.4)
///         From: \"Guest-Host-Communication Interface (GHCI) for Intel
///         Trust Domain Extensions (Intel TDX)\".
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSvkl {
    pub Header: FfiAcpiTableHeader,
    pub Count: u32,
}
///  SVKL - Storage Volume Key Location Table (ACPI 6.4)
///         From: \"Guest-Host-Communication Interface (GHCI) for Intel
///         Trust Domain Extensions (Intel TDX)\".
///         Version 1
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiSvklKey {
    pub Type: u16,
    pub Format: u16,
    pub Size: u32,
    pub Address: u64,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiSvklType {
    ACPI_SVKL_TYPE_MAIN_STORAGE = 0,
    ACPI_SVKL_TYPE_RESERVED = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiSvklFormat {
    ACPI_SVKL_FORMAT_RAW_BINARY = 0,
    ACPI_SVKL_FORMAT_RESERVED = 1,
}
///  SLIC - Software Licensing Description Table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSlic {
    pub Header: FfiAcpiTableHeader,
}
///  SLIC - Software Licensing Description Table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 

///  SLIT - System Locality Distance Information Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSlit {
    pub Header: FfiAcpiTableHeader,
    pub LocalityCount: u64,
    pub Entry: [u8; 1usize],
}
///  SLIT - System Locality Distance Information Table
///         Version 1
/// 

///  SPCR - Serial Port Console Redirection table
///         Version 2
/// 
///  Conforms to \"Serial Port Console Redirection Table\",
///  Version 1.03, August 10, 2015
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSpcr {
    pub Header: FfiAcpiTableHeader,
    pub InterfaceType: u8,
    pub Reserved: [u8; 3usize],
    pub SerialPort: AcpiGenericAddress,
    pub InterruptType: u8,
    pub PcInterrupt: u8,
    pub Interrupt: u32,
    pub BaudRate: u8,
    pub Parity: u8,
    pub StopBits: u8,
    pub FlowControl: u8,
    pub TerminalType: u8,
    pub Reserved1: u8,
    pub PciDeviceId: u16,
    pub PciVendorId: u16,
    pub PciBus: u8,
    pub PciDevice: u8,
    pub PciFunction: u8,
    pub PciFlags: u32,
    pub PciSegment: u8,
    pub Reserved2: u32,
}
///  SPCR - Serial Port Console Redirection table
///         Version 2
/// 
///  Conforms to \"Serial Port Console Redirection Table\",
///  Version 1.03, August 10, 2015
/// 

///  SPMI - Server Platform Management Interface table
///         Version 5
/// 
///  Conforms to \"Intelligent Platform Management Interface Specification
///  Second Generation v2.0\", Document Revision 1.0, February 12, 2004 with
///  June 12, 2009 markup.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableSpmi {
    pub Header: FfiAcpiTableHeader,
    pub InterfaceType: u8,
    pub Reserved: u8,
    pub SpecRevision: u16,
    pub InterruptType: u8,
    pub GpeNumber: u8,
    pub Reserved1: u8,
    pub PciDeviceFlag: u8,
    pub Interrupt: u32,
    pub IpmiRegister: AcpiGenericAddress,
    pub PciSegment: u8,
    pub PciBus: u8,
    pub PciDevice: u8,
    pub PciFunction: u8,
    pub Reserved2: u8,
}
///  SPMI - Server Platform Management Interface table
///         Version 5
/// 
///  Conforms to \"Intelligent Platform Management Interface Specification
///  Second Generation v2.0\", Document Revision 1.0, February 12, 2004 with
///  June 12, 2009 markup.
/// 

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiSpmiInterfaceTypes {
    ACPI_SPMI_NOT_USED = 0,
    ACPI_SPMI_KEYBOARD = 1,
    ACPI_SPMI_SMI = 2,
    ACPI_SPMI_BLOCK_TRANSFER = 3,
    ACPI_SPMI_SMBUS = 4,
    ACPI_SPMI_RESERVED = 5,
}


///  STAO - Status Override Table (_STA override) - ACPI 6.0
///         Version 1
/// 
///  Conforms to \"ACPI Specification for Status Override Table\"
///  6 January 2015
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableStao {
    pub Header: FfiAcpiTableHeader,
    pub IgnoreUart: u8,
}
///  STAO - Status Override Table (_STA override) - ACPI 6.0
///         Version 1
/// 
///  Conforms to \"ACPI Specification for Status Override Table\"
///  6 January 2015
/// 


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableTcpaClient {
    pub MinimumLogLength: u32,
    pub LogAddress: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableTcpaServer {
    pub Reserved: u16,
    pub MinimumLogLength: u64,
    pub LogAddress: u64,
    pub SpecRevision: u16,
    pub DeviceFlags: u8,
    pub InterruptFlags: u8,
    pub GpeNumber: u8,
    pub Reserved2: [u8; 3usize],
    pub GlobalInterrupt: u32,
    pub Address: AcpiGenericAddress,
    pub Reserved3: u32,
    pub ConfigAddress: AcpiGenericAddress,
    pub Group: u8,
    pub Bus: u8,
    pub Device: u8,
    pub Function: u8,
}


///  UEFI - UEFI Boot optimization Table
///         Version 1
/// 
///  Conforms to \"Unified Extensible Firmware Interface Specification\",
///  Version 2.3, May 8, 2009
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableUefi {
    pub Header: FfiAcpiTableHeader,
    pub Identifier: [u8; 16usize],
    pub DataOffset: u16,
}
///  UEFI - UEFI Boot optimization Table
///         Version 1
/// 
///  Conforms to \"Unified Extensible Firmware Interface Specification\",
///  Version 2.3, May 8, 2009
/// 



///  XENV - Xen Environment Table (ACPI 6.0)
///         Version 1
/// 
///  Conforms to \"ACPI Specification for Xen Environment Table\" 4 January 2015
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableXenv {
    pub Header: FfiAcpiTableHeader,
    pub GrantTableAddress: u64,
    pub GrantTableSize: u64,
    pub EventInterrupt: u32,
    pub EventFlags: u8,
}