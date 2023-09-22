use crate::bindings::types::FfiAcpiTableHeader;

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
    pub Header: FfiAcpiTableHeader,
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
    pub Header: FfiAcpiTableHeader,
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

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_tmp23_trailer {
    pub Reserved: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_table_tpm2 {
    pub Header: FfiAcpiTableHeader,
    pub PlatformClass: u16,
    pub Reserved: u16,
    pub ControlAddress: u64,
    pub StartMethod: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_tpm2_trailer {
    pub MethodParameters: [u8; 12usize],
    pub MinimumLogLength: u32,
    pub LogAddress: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_tpm2_arm_smc {
    pub GlobalInterrupt: u32,
    pub InterruptFlags: u8,
    pub OperationFlags: u8,
    pub Reserved: u16,
    pub FunctionId: u32,
}
