use crate::types::AcpiTableHeader;

///  TCPA - Trusted Computing Platform Alliance table
///         Version 2
/// 
///  TCG Hardware Interface Table for TPM 1.2 Clients and Servers
/// 
///  Conforms to \"TCG ACPI Specification, Family 1.2 and 2.0\",
///  Version 1.2, Revision 8
///  February 27, 2017
/// 
///  NOTE: There are two versions of the table with the same signature --
///  the client version and the server version. The common PlatformClass
///  field is used to differentiate the two types of tables.
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_tcpa_hdr {
    pub Header: AcpiTableHeader,
    pub PlatformClass: u16,
}
///  TCPA - Trusted Computing Platform Alliance table
///         Version 2
/// 
///  TCG Hardware Interface Table for TPM 1.2 Clients and Servers
/// 
///  Conforms to \"TCG ACPI Specification, Family 1.2 and 2.0\",
///  Version 1.2, Revision 8
///  February 27, 2017
/// 
///  NOTE: There are two versions of the table with the same signature --
///  the client version and the server version. The common PlatformClass
///  field is used to differentiate the two types of tables.
/// 
pub type ACPI_TABLE_TCPA_HDR = acpi_table_tcpa_hdr;

///  TPM2 - Trusted Platform Module (TPM) 2.0 Hardware Interface Table
///         Version 4
/// 
///  TCG Hardware Interface Table for TPM 2.0 Clients and Servers
/// 
///  Conforms to \"TCG ACPI Specification, Family 1.2 and 2.0\",
///  Version 1.2, Revision 8
///  February 27, 2017
/// 
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_tpm23 {
    pub Header: AcpiTableHeader,
    pub Reserved: u32,
    pub ControlAddress: u64,
    pub StartMethod: u32,
}
///  TPM2 - Trusted Platform Module (TPM) 2.0 Hardware Interface Table
///         Version 4
/// 
///  TCG Hardware Interface Table for TPM 2.0 Clients and Servers
/// 
///  Conforms to \"TCG ACPI Specification, Family 1.2 and 2.0\",
///  Version 1.2, Revision 8
///  February 27, 2017
/// 
pub type ACPI_TABLE_TPM23 = acpi_table_tpm23;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_tmp23_trailer {
    pub Reserved: u32,
}
pub type ACPI_TPM23_TRAILER = acpi_tmp23_trailer;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_tpm2 {
    pub Header: AcpiTableHeader,
    pub PlatformClass: u16,
    pub Reserved: u16,
    pub ControlAddress: u64,
    pub StartMethod: u32,
}
pub type ACPI_TABLE_TPM2 = acpi_table_tpm2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_tpm2_trailer {
    pub MethodParameters: [u8; 12usize],
    pub MinimumLogLength: u32,
    pub LogAddress: u64,
}
pub type ACPI_TPM2_TRAILER = acpi_tpm2_trailer;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_tpm2_arm_smc {
    pub GlobalInterrupt: u32,
    pub InterruptFlags: u8,
    pub OperationFlags: u8,
    pub Reserved: u16,
    pub FunctionId: u32,
}
pub type ACPI_TPM2_ARM_SMC = acpi_tpm2_arm_smc;