use crate::{types::AcpiTableHeader, bindings::types::__IncompleteArrayField, interface::AcpiGenericAddress};

use super::ACPI_SUBTABLE_HEADER;

///  ASF - Alert Standard Format table (Signature \"ASF!\")
///        Revision 0x10
/// 
///  Conforms to the Alert Standard Format Specification V2.0, 23 April 2003
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_asf {
    pub Header: AcpiTableHeader,
}
///  ASF - Alert Standard Format table (Signature \"ASF!\")
///        Revision 0x10
/// 
///  Conforms to the Alert Standard Format Specification V2.0, 23 April 2003
/// 
pub type ACPI_TABLE_ASF = acpi_table_asf;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_header {
    pub Type: u8,
    pub Reserved: u8,
    pub Length: u16,
}
pub type ACPI_ASF_HEADER = acpi_asf_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiAsfType {
    ACPI_ASF_TYPE_INFO = 0,
    ACPI_ASF_TYPE_ALERT = 1,
    ACPI_ASF_TYPE_CONTROL = 2,
    ACPI_ASF_TYPE_BOOT = 3,
    ACPI_ASF_TYPE_ADDRESS = 4,
    ACPI_ASF_TYPE_RESERVED = 5,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_info {
    pub Header: ACPI_ASF_HEADER,
    pub MinResetValue: u8,
    pub MinPollInterval: u8,
    pub SystemId: u16,
    pub MfgId: u32,
    pub Flags: u8,
    pub Reserved2: [u8; 3usize],
}
pub type ACPI_ASF_INFO = acpi_asf_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_alert {
    pub Header: ACPI_ASF_HEADER,
    pub AssertMask: u8,
    pub DeassertMask: u8,
    pub Alerts: u8,
    pub DataLength: u8,
}
pub type ACPI_ASF_ALERT = acpi_asf_alert;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_alert_data {
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
pub type ACPI_ASF_ALERT_DATA = acpi_asf_alert_data;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_remote {
    pub Header: ACPI_ASF_HEADER,
    pub Controls: u8,
    pub DataLength: u8,
    pub Reserved2: u16,
}
pub type ACPI_ASF_REMOTE = acpi_asf_remote;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_control_data {
    pub Function: u8,
    pub Address: u8,
    pub Command: u8,
    pub Value: u8,
}
pub type ACPI_ASF_CONTROL_DATA = acpi_asf_control_data;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_rmcp {
    pub Header: ACPI_ASF_HEADER,
    pub Capabilities: [u8; 7usize],
    pub CompletionCode: u8,
    pub EnterpriseId: u32,
    pub Command: u8,
    pub Parameter: u16,
    pub BootOptions: u16,
    pub OemParameters: u16,
}
pub type ACPI_ASF_RMCP = acpi_asf_rmcp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_asf_address {
    pub Header: ACPI_ASF_HEADER,
    pub EpromAddress: u8,
    pub Devices: u8,
}
pub type ACPI_ASF_ADDRESS = acpi_asf_address;
///  BERT - Boot Error Record Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_bert {
    pub Header: AcpiTableHeader,
    pub RegionLength: u32,
    pub Address: u64,
}
///  BERT - Boot Error Record Table (ACPI 4.0)
///         Version 1
/// 
pub type ACPI_TABLE_BERT = acpi_table_bert;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_bert_region {
    pub BlockStatus: u32,
    pub RawDataOffset: u32,
    pub RawDataLength: u32,
    pub DataLength: u32,
    pub ErrorSeverity: u32,
}
pub type ACPI_BERT_REGION = acpi_bert_region;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiBertErrorSeverity {
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
pub struct acpi_table_bgrt {
    pub Header: AcpiTableHeader,
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
pub type ACPI_TABLE_BGRT = acpi_table_bgrt;
///  BOOT - Simple Boot Flag Table
///         Version 1
/// 
///  Conforms to the \"Simple Boot Flag Specification\", Version 2.1
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_boot {
    pub Header: AcpiTableHeader,
    pub CmosIndex: u8,
    pub Reserved: [u8; 3usize],
}
///  BOOT - Simple Boot Flag Table
///         Version 1
/// 
///  Conforms to the \"Simple Boot Flag Specification\", Version 2.1
/// 
pub type ACPI_TABLE_BOOT = acpi_table_boot;

///  CPEP - Corrected Platform Error Polling table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_cpep {
    pub Header: AcpiTableHeader,
    pub Reserved: u64,
}
///  CPEP - Corrected Platform Error Polling table (ACPI 4.0)
///         Version 1
/// 
pub type ACPI_TABLE_CPEP = acpi_table_cpep;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_cpep_polling {
    pub Header: ACPI_SUBTABLE_HEADER,
    pub Id: u8,
    pub Eid: u8,
    pub Interval: u32,
}
pub type ACPI_CPEP_POLLING = acpi_cpep_polling;


///  DBGP - Debug Port table
///         Version 1
/// 
///  Conforms to the \"Debug Port Specification\", Version 1.00, 2/9/2000
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_dbgp {
    pub Header: AcpiTableHeader,
    pub Type: u8,
    pub Reserved: [u8; 3usize],
    pub DebugPort: AcpiGenericAddress,
}
///  DBGP - Debug Port table
///         Version 1
/// 
///  Conforms to the \"Debug Port Specification\", Version 1.00, 2/9/2000
/// 
pub type ACPI_TABLE_DBGP = acpi_table_dbgp;


///  ECDT - Embedded Controller Boot Resources Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_ecdt {
    pub Header: AcpiTableHeader,
    pub Control: AcpiGenericAddress,
    pub Data: AcpiGenericAddress,
    pub Uid: u32,
    pub Gpe: u8,
    pub Id: [u8; 1usize],
}
///  ECDT - Embedded Controller Boot Resources Table
///         Version 1
/// 
pub type ACPI_TABLE_ECDT = acpi_table_ecdt;


///  BDAT - BIOS Data ACPI Table
/// 
///  Conforms to \"BIOS Data ACPI Table\", Interface Specification v4.0 Draft 5
///  Nov 2020
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_bdat {
    pub Header: AcpiTableHeader,
    pub Gas: AcpiGenericAddress,
}
///  BDAT - BIOS Data ACPI Table
/// 
///  Conforms to \"BIOS Data ACPI Table\", Interface Specification v4.0 Draft 5
///  Nov 2020
/// 
pub type ACPI_TABLE_BDAT = acpi_table_bdat;



///  LPIT - Low Power Idle Table
/// 
///  Conforms to \"ACPI Low Power Idle Table (LPIT)\" July 2014.
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_lpit {
    pub Header: AcpiTableHeader,
}
///  LPIT - Low Power Idle Table
/// 
///  Conforms to \"ACPI Low Power Idle Table (LPIT)\" July 2014.
/// 
pub type ACPI_TABLE_LPIT = acpi_table_lpit;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_lpit_header {
    pub Type: u32,
    pub Length: u32,
    pub UniqueId: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
pub type ACPI_LPIT_HEADER = acpi_lpit_header;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiLpitType {
    ACPI_LPIT_TYPE_NATIVE_CSTATE = 0,
    ACPI_LPIT_TYPE_RESERVED = 1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_lpit_native {
    pub Header: ACPI_LPIT_HEADER,
    pub EntryTrigger: AcpiGenericAddress,
    pub Residency: u32,
    pub Latency: u32,
    pub ResidencyCounter: AcpiGenericAddress,
    pub CounterFrequency: u64,
}
pub type ACPI_LPIT_NATIVE = acpi_lpit_native;

///  MCFG - PCI Memory Mapped Configuration table and subtable
///         Version 1
/// 
///  Conforms to \"PCI Firmware Specification\", Revision 3.0, June 20, 2005
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_mcfg {
    pub Header: AcpiTableHeader,
    pub Reserved: [u8; 8usize],
}
///  MCFG - PCI Memory Mapped Configuration table and subtable
///         Version 1
/// 
///  Conforms to \"PCI Firmware Specification\", Revision 3.0, June 20, 2005
/// 
pub type ACPI_TABLE_MCFG = acpi_table_mcfg;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_mcfg_allocation {
    pub Address: u64,
    pub PciSegment: u16,
    pub StartBusNumber: u8,
    pub EndBusNumber: u8,
    pub Reserved: u32,
}
pub type ACPI_MCFG_ALLOCATION = acpi_mcfg_allocation;
///  MCHI - Management Controller Host Interface Table
///         Version 1
/// 
///  Conforms to \"Management Component Transport Protocol (MCTP) Host
///  Interface Specification\", Revision 1.0.0a, October 13, 2009
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_mchi {
    pub Header: AcpiTableHeader,
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
pub type ACPI_TABLE_MCHI = acpi_table_mchi;

///  MSCT - Maximum System Characteristics Table (ACPI 4.0)
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_msct {
    pub Header: AcpiTableHeader,
    pub ProximityOffset: u32,
    pub MaxProximityDomains: u32,
    pub MaxClockDomains: u32,
    pub MaxAddress: u64,
}
///  MSCT - Maximum System Characteristics Table (ACPI 4.0)
///         Version 1
/// 
pub type ACPI_TABLE_MSCT = acpi_table_msct;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_msct_proximity {
    pub Revision: u8,
    pub Length: u8,
    pub RangeStart: u32,
    pub RangeEnd: u32,
    pub ProcessorCapacity: u32,
    pub MemoryCapacity: u64,
}
pub type ACPI_MSCT_PROXIMITY = acpi_msct_proximity;
///  MSDM - Microsoft Data Management table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_msdm {
    pub Header: AcpiTableHeader,
}
///  MSDM - Microsoft Data Management table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 
pub type ACPI_TABLE_MSDM = acpi_table_msdm;



///  PDTT - Platform Debug Trigger Table (ACPI 6.2)
///         Version 0
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_pdtt {
    pub Header: AcpiTableHeader,
    pub TriggerCount: u8,
    pub Reserved: [u8; 3usize],
    pub ArrayOffset: u32,
}
///  PDTT - Platform Debug Trigger Table (ACPI 6.2)
///         Version 0
/// 
pub type ACPI_TABLE_PDTT = acpi_table_pdtt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_pdtt_channel {
    pub SubchannelId: u8,
    pub Flags: u8,
}
pub type ACPI_PDTT_CHANNEL = acpi_pdtt_channel;


///  RGRT - Regulatory Graphics Resource Table
///         Version 1
/// 
///  Conforms to \"ACPI RGRT\" available at:
///  https://microsoft.github.io/mu/dyn/mu_plus/MsCorePkg/AcpiRGRT/feature_acpi_rgrt/
/// 
#[repr(C, packed)]
pub struct acpi_table_rgrt {
    pub Header: AcpiTableHeader,
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
pub type ACPI_TABLE_RGRT = acpi_table_rgrt;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiRgrtImageType {
    ACPI_RGRT_TYPE_RESERVED0 = 0,
    ACPI_RGRT_IMAGE_TYPE_PNG = 1,
    ACPI_RGRT_TYPE_RESERVED = 2,
}
///  SBST - Smart Battery Specification Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_sbst {
    pub Header: AcpiTableHeader,
    pub WarningLevel: u32,
    pub LowLevel: u32,
    pub CriticalLevel: u32,
}
///  SBST - Smart Battery Specification Table
///         Version 1
/// 
pub type ACPI_TABLE_SBST = acpi_table_sbst;
///  SDEI - Software Delegated Exception Interface Descriptor Table
/// 
///  Conforms to \"Software Delegated Exception Interface (SDEI)\" ARM DEN0054A,
///  May 8th, 2017. Copyright 2017 ARM Ltd.
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_sdei {
    pub Header: AcpiTableHeader,
}
///  SDEI - Software Delegated Exception Interface Descriptor Table
/// 
///  Conforms to \"Software Delegated Exception Interface (SDEI)\" ARM DEN0054A,
///  May 8th, 2017. Copyright 2017 ARM Ltd.
/// 
pub type ACPI_TABLE_SDEI = acpi_table_sdei;


///  SVKL - Storage Volume Key Location Table (ACPI 6.4)
///         From: \"Guest-Host-Communication Interface (GHCI) for Intel
///         Trust Domain Extensions (Intel TDX)\".
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_svkl {
    pub Header: AcpiTableHeader,
    pub Count: u32,
}
///  SVKL - Storage Volume Key Location Table (ACPI 6.4)
///         From: \"Guest-Host-Communication Interface (GHCI) for Intel
///         Trust Domain Extensions (Intel TDX)\".
///         Version 1
/// 
pub type ACPI_TABLE_SVKL = acpi_table_svkl;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_svkl_key {
    pub Type: u16,
    pub Format: u16,
    pub Size: u32,
    pub Address: u64,
}
pub type ACPI_SVKL_KEY = acpi_svkl_key;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum acpi_svkl_type {
    ACPI_SVKL_TYPE_MAIN_STORAGE = 0,
    ACPI_SVKL_TYPE_RESERVED = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum acpi_svkl_format {
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
pub struct acpi_table_slic {
    pub Header: AcpiTableHeader,
}
///  SLIC - Software Licensing Description Table
/// 
///  Conforms to \"Microsoft Software Licensing Tables (SLIC and MSDM)\",
///  November 29, 2011. Copyright 2011 Microsoft
/// 
pub type ACPI_TABLE_SLIC = acpi_table_slic;
///  SLIT - System Locality Distance Information Table
///         Version 1
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_slit {
    pub Header: AcpiTableHeader,
    pub LocalityCount: u64,
    pub Entry: [u8; 1usize],
}
///  SLIT - System Locality Distance Information Table
///         Version 1
/// 
pub type ACPI_TABLE_SLIT = acpi_table_slit;
///  SPCR - Serial Port Console Redirection table
///         Version 2
/// 
///  Conforms to \"Serial Port Console Redirection Table\",
///  Version 1.03, August 10, 2015
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_spcr {
    pub Header: AcpiTableHeader,
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
pub type ACPI_TABLE_SPCR = acpi_table_spcr;
///  SPMI - Server Platform Management Interface table
///         Version 5
/// 
///  Conforms to \"Intelligent Platform Management Interface Specification
///  Second Generation v2.0\", Document Revision 1.0, February 12, 2004 with
///  June 12, 2009 markup.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_spmi {
    pub Header: AcpiTableHeader,
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
pub type ACPI_TABLE_SPMI = acpi_table_spmi;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AcpiSpmiInterfaceTypes {
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
pub struct acpi_table_stao {
    pub Header: AcpiTableHeader,
    pub IgnoreUart: u8,
}
///  STAO - Status Override Table (_STA override) - ACPI 6.0
///         Version 1
/// 
///  Conforms to \"ACPI Specification for Status Override Table\"
///  6 January 2015
/// 
pub type ACPI_TABLE_STAO = acpi_table_stao;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_tcpa_client {
    pub MinimumLogLength: u32,
    pub LogAddress: u64,
}
pub type ACPI_TABLE_TCPA_CLIENT = acpi_table_tcpa_client;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_tcpa_server {
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
pub type ACPI_TABLE_TCPA_SERVER = acpi_table_tcpa_server;

///  UEFI - UEFI Boot optimization Table
///         Version 1
/// 
///  Conforms to \"Unified Extensible Firmware Interface Specification\",
///  Version 2.3, May 8, 2009
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_uefi {
    pub Header: AcpiTableHeader,
    pub Identifier: [u8; 16usize],
    pub DataOffset: u16,
}
///  UEFI - UEFI Boot optimization Table
///         Version 1
/// 
///  Conforms to \"Unified Extensible Firmware Interface Specification\",
///  Version 2.3, May 8, 2009
/// 
pub type ACPI_TABLE_UEFI = acpi_table_uefi;


///  XENV - Xen Environment Table (ACPI 6.0)
///         Version 1
/// 
///  Conforms to \"ACPI Specification for Xen Environment Table\" 4 January 2015
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_xenv {
    pub Header: AcpiTableHeader,
    pub GrantTableAddress: u64,
    pub GrantTableSize: u64,
    pub EventInterrupt: u32,
    pub EventFlags: u8,
}
///  XENV - Xen Environment Table (ACPI 6.0)
///         Version 1
/// 
///  Conforms to \"ACPI Specification for Xen Environment Table\" 4 January 2015
/// 
pub type ACPI_TABLE_XENV = acpi_table_xenv;