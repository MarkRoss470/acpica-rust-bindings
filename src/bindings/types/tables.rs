// use super::*;
// pub mod aest;
// pub mod cedt;
// pub mod csrt;
// pub mod dbg2;
// pub mod dmar;
// pub mod drtm;
// pub mod einj;
// pub mod erst;
pub mod fadt;
// pub mod fpdt;
// pub mod gtdt;
// pub mod hest;
// pub mod hmat;
// pub mod hpet;
// pub mod ibft;
// pub mod iort;
// pub mod ivrs;
// pub mod madt;
// pub mod misc;
// pub mod mpst;
// pub mod nfit;
// pub mod pcct;
// pub mod phat;
// pub mod pmtt;
// pub mod pptt;
// pub mod prmt;
// pub mod rasf;
// pub mod rsdt;
// pub mod sdev;
// pub mod srat;
// pub mod trusted_platform;
// pub mod viot;
// pub mod watchdog;
// pub mod windows;

#[doc = " Master ACPI Table Header. This common header is used by all ACPI tables"]
#[doc = " except the RSDP and FACS."]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FfiAcpiTableHeader {
    pub Signature: [i8; 4usize],
    pub Length: u32,
    pub Revision: u8,
    pub Checksum: u8,
    pub OemId: [i8; 6usize],
    pub OemTableId: [i8; 8usize],
    pub OemRevision: u32,
    pub AslCompilerId: [i8; 4usize],
    pub AslCompilerRevision: u32,
}

#[doc = " Common subtable headers"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_subtable_header {
    pub Type: u8,
    pub Length: u8,
}
#[doc = " Common subtable headers"]
#[doc = ""]
pub type ACPI_SUBTABLE_HEADER = acpi_subtable_header;

#[doc = " RSDP - Root System Description Pointer (Signature is \"RSD PTR \")"]
#[doc = "        Version 2"]
#[doc = ""]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_rsdp {
    pub Signature: [i8; 8usize],
    pub Checksum: u8,
    pub OemId: [i8; 6usize],
    pub Revision: u8,
    pub RsdtPhysicalAddress: u32,
    pub Length: u32,
    pub XsdtPhysicalAddress: u64,
    pub ExtendedChecksum: u8,
    pub Reserved: [u8; 3usize],
}
#[doc = " RSDP - Root System Description Pointer (Signature is \"RSD PTR \")"]
#[doc = "        Version 2"]
#[doc = ""]
pub type ACPI_TABLE_RSDP = acpi_table_rsdp;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_rsdp_common {
    pub Signature: [i8; 8usize],
    pub Checksum: u8,
    pub OemId: [i8; 6usize],
    pub Revision: u8,
    pub RsdtPhysicalAddress: u32,
}
pub type ACPI_RSDP_COMMON = acpi_rsdp_common;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_rsdp_extension {
    pub Length: u32,
    pub XsdtPhysicalAddress: u64,
    pub ExtendedChecksum: u8,
    pub Reserved: [u8; 3usize],
}
pub type ACPI_RSDP_EXTENSION = acpi_rsdp_extension;
