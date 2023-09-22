use crate::bindings::types::FfiAcpiTableHeader;

///  IBFT - Boot Firmware Table
///         Version 1
/// 
///  Conforms to \"iSCSI Boot Firmware Table (iBFT) as Defined in ACPI 3.0b
///  Specification\", Version 1.01, March 1, 2007
/// 
///  Note: It appears that this table is not intended to appear in the RSDT/XSDT.
///  Therefore, it is not currently supported by the disassembler.
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableIbft {
    pub Header: FfiAcpiTableHeader,
    pub Reserved: [u8; 12usize],
}
///  IBFT - Boot Firmware Table
///         Version 1
/// 
///  Conforms to \"iSCSI Boot Firmware Table (iBFT) as Defined in ACPI 3.0b
///  Specification\", Version 1.01, March 1, 2007
/// 
///  Note: It appears that this table is not intended to appear in the RSDT/XSDT.
///  Therefore, it is not currently supported by the disassembler.
/// 

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIbftHeader {
    pub Type: u8,
    pub Version: u8,
    pub Length: u16,
    pub Index: u8,
    pub Flags: u8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiAcpiIbftType {
    ACPI_IBFT_TYPE_NOT_USED = 0,
    ACPI_IBFT_TYPE_CONTROL = 1,
    ACPI_IBFT_TYPE_INITIATOR = 2,
    ACPI_IBFT_TYPE_NIC = 3,
    ACPI_IBFT_TYPE_TARGET = 4,
    ACPI_IBFT_TYPE_EXTENSIONS = 5,
    ACPI_IBFT_TYPE_RESERVED = 6,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIbftControl {
    pub Header: FfiAcpiIbftHeader,
    pub Extensions: u16,
    pub InitiatorOffset: u16,
    pub Nic0Offset: u16,
    pub Target0Offset: u16,
    pub Nic1Offset: u16,
    pub Target1Offset: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIbftInitiator {
    pub Header: FfiAcpiIbftHeader,
    pub SnsServer: [u8; 16usize],
    pub SlpServer: [u8; 16usize],
    pub PrimaryServer: [u8; 16usize],
    pub SecondaryServer: [u8; 16usize],
    pub NameLength: u16,
    pub NameOffset: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIbftNic {
    pub Header: FfiAcpiIbftHeader,
    pub IpAddress: [u8; 16usize],
    pub SubnetMaskPrefix: u8,
    pub Origin: u8,
    pub Gateway: [u8; 16usize],
    pub PrimaryDns: [u8; 16usize],
    pub SecondaryDns: [u8; 16usize],
    pub Dhcp: [u8; 16usize],
    pub Vlan: u16,
    pub MacAddress: [u8; 6usize],
    pub PciAddress: u16,
    pub NameLength: u16,
    pub NameOffset: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiIbftTarget {
    pub Header: FfiAcpiIbftHeader,
    pub TargetIpAddress: [u8; 16usize],
    pub TargetIpSocket: u16,
    pub TargetBootLun: [u8; 8usize],
    pub ChapType: u8,
    pub NicAssociation: u8,
    pub TargetNameLength: u16,
    pub TargetNameOffset: u16,
    pub ChapNameLength: u16,
    pub ChapNameOffset: u16,
    pub ChapSecretLength: u16,
    pub ChapSecretOffset: u16,
    pub ReverseChapNameLength: u16,
    pub ReverseChapNameOffset: u16,
    pub ReverseChapSecretLength: u16,
    pub ReverseChapSecretOffset: u16,
}
