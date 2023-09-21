// Adapted from source/include/acexcep.h

use core::fmt::Display;

#[repr(transparent)]
pub(crate) struct AcpiStatus(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub enum AcpiError {
    /*
     * Environmental exceptions
     */
    Error = 0x0001 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NoAcpiTables = 0x0002 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NoNamespace = 0x0003 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NoMemory = 0x0004 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NotFound = 0x0005 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NotExist = 0x0006 | AcpiError::AE_CODE_ENVIRONMENTAL,
    AlreadyExists = 0x0007 | AcpiError::AE_CODE_ENVIRONMENTAL,
    Type = 0x0008 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NullObject = 0x0009 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NullEntry = 0x000A | AcpiError::AE_CODE_ENVIRONMENTAL,
    BufferOverflow = 0x000B | AcpiError::AE_CODE_ENVIRONMENTAL,
    StackOverflow = 0x000C | AcpiError::AE_CODE_ENVIRONMENTAL,
    StackUnderflow = 0x000D | AcpiError::AE_CODE_ENVIRONMENTAL,
    NotImplemented = 0x000E | AcpiError::AE_CODE_ENVIRONMENTAL,
    Support = 0x000F | AcpiError::AE_CODE_ENVIRONMENTAL,
    Limit = 0x0010 | AcpiError::AE_CODE_ENVIRONMENTAL,
    Time = 0x0011 | AcpiError::AE_CODE_ENVIRONMENTAL,
    AcquireDeadlock = 0x0012 | AcpiError::AE_CODE_ENVIRONMENTAL,
    ReleaseDeadlock = 0x0013 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NotAcquired = 0x0014 | AcpiError::AE_CODE_ENVIRONMENTAL,
    AlreadyAcquired = 0x0015 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NoHardwareResponse = 0x0016 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NoGlobalLock = 0x0017 | AcpiError::AE_CODE_ENVIRONMENTAL,
    AbortMethod = 0x0018 | AcpiError::AE_CODE_ENVIRONMENTAL,
    SameHandler = 0x0019 | AcpiError::AE_CODE_ENVIRONMENTAL,
    NoHandler = 0x001A | AcpiError::AE_CODE_ENVIRONMENTAL,
    OwnerIdLimit = 0x001B | AcpiError::AE_CODE_ENVIRONMENTAL,
    NotConfigured = 0x001C | AcpiError::AE_CODE_ENVIRONMENTAL,
    Access = 0x001D | AcpiError::AE_CODE_ENVIRONMENTAL,
    IoError = 0x001E | AcpiError::AE_CODE_ENVIRONMENTAL,
    NumericOverflow = 0x001F | AcpiError::AE_CODE_ENVIRONMENTAL,
    HexOverflow = 0x0020 | AcpiError::AE_CODE_ENVIRONMENTAL,
    DecimalOverflow = 0x0021 | AcpiError::AE_CODE_ENVIRONMENTAL,
    OctalOverflow = 0x0022 | AcpiError::AE_CODE_ENVIRONMENTAL,
    EndOfTable = 0x0023 | AcpiError::AE_CODE_ENVIRONMENTAL,

    /*
     * Programmer exceptions
     */
    BadParameter = 0x0001 | AcpiError::AE_CODE_PROGRAMMER,
    BadCharacter = 0x0002 | AcpiError::AE_CODE_PROGRAMMER,
    BadPathname = 0x0003 | AcpiError::AE_CODE_PROGRAMMER,
    BadData = 0x0004 | AcpiError::AE_CODE_PROGRAMMER,
    BadHexConstant = 0x0005 | AcpiError::AE_CODE_PROGRAMMER,
    BadOctalConstant = 0x0006 | AcpiError::AE_CODE_PROGRAMMER,
    BadDecimalConstant = 0x0007 | AcpiError::AE_CODE_PROGRAMMER,
    MissingArguments = 0x0008 | AcpiError::AE_CODE_PROGRAMMER,
    BadAddress = 0x0009 | AcpiError::AE_CODE_PROGRAMMER,

    /*
     * Acpi table exceptions
     */
    BadSignature = 0x0001 | AcpiError::AE_CODE_ACPI_TABLES,
    BadHeader = 0x0002 | AcpiError::AE_CODE_ACPI_TABLES,
    BadChecksum = 0x0003 | AcpiError::AE_CODE_ACPI_TABLES,
    BadValue = 0x0004 | AcpiError::AE_CODE_ACPI_TABLES,
    InvalidTableLength = 0x0005 | AcpiError::AE_CODE_ACPI_TABLES,

    /*
     * AML exceptions. These are caused by problems with
     * the actual AML byte stream
     */
    AmlBadOpcode = 0x0001 | AcpiError::AE_CODE_AML,
    AmlNoOperand = 0x0002 | AcpiError::AE_CODE_AML,
    AmlOperandType = 0x0003 | AcpiError::AE_CODE_AML,
    AmlOperandValue = 0x0004 | AcpiError::AE_CODE_AML,
    AmlUninitializedLocal = 0x0005 | AcpiError::AE_CODE_AML,
    AmlUninitializedArg = 0x0006 | AcpiError::AE_CODE_AML,
    AmlUninitializedElement = 0x0007 | AcpiError::AE_CODE_AML,
    AmlNumericOverflow = 0x0008 | AcpiError::AE_CODE_AML,
    AmlRegionLimit = 0x0009 | AcpiError::AE_CODE_AML,
    AmlBufferLimit = 0x000A | AcpiError::AE_CODE_AML,
    AmlPackageLimit = 0x000B | AcpiError::AE_CODE_AML,
    AmlDivideByZero = 0x000C | AcpiError::AE_CODE_AML,
    AmlBadName = 0x000D | AcpiError::AE_CODE_AML,
    AmlNameNotFound = 0x000E | AcpiError::AE_CODE_AML,
    AmlInternal = 0x000F | AcpiError::AE_CODE_AML,
    AmlInvalidSpaceId = 0x0010 | AcpiError::AE_CODE_AML,
    AmlStringLimit = 0x0011 | AcpiError::AE_CODE_AML,
    AmlNoReturnValue = 0x0012 | AcpiError::AE_CODE_AML,
    AmlMethodLimit = 0x0013 | AcpiError::AE_CODE_AML,
    AmlNotOwner = 0x0014 | AcpiError::AE_CODE_AML,
    AmlMutexOrder = 0x0015 | AcpiError::AE_CODE_AML,
    AmlMutexNotAcquired = 0x0016 | AcpiError::AE_CODE_AML,
    AmlInvalidResourceType = 0x0017 | AcpiError::AE_CODE_AML,
    AmlInvalidIndex = 0x0018 | AcpiError::AE_CODE_AML,
    AmlRegisterLimit = 0x0019 | AcpiError::AE_CODE_AML,
    AmlNoWhile = 0x001A | AcpiError::AE_CODE_AML,
    AmlAlignment = 0x001B | AcpiError::AE_CODE_AML,
    AmlNoResourceEndTag = 0x001C | AcpiError::AE_CODE_AML,
    AmlBadResourceValue = 0x001D | AcpiError::AE_CODE_AML,
    AmlCircularReference = 0x001E | AcpiError::AE_CODE_AML,
    AmlBadResourceLength = 0x001F | AcpiError::AE_CODE_AML,
    AmlIllegalAddress = 0x0020 | AcpiError::AE_CODE_AML,
    AmlLoopTimeout = 0x0021 | AcpiError::AE_CODE_AML,
    AmlUninitializedNode = 0x0022 | AcpiError::AE_CODE_AML,
    AmlTargetType = 0x0023 | AcpiError::AE_CODE_AML,
    AmlProtocol = 0x0024 | AcpiError::AE_CODE_AML,
    AmlBufferLength = 0x0025 | AcpiError::AE_CODE_AML,

    /*
     * Internal exceptions used for control
     */
    ControlReturnValue = 0x0001 | AcpiError::AE_CODE_CONTROL,
    ControlPending = 0x0002 | AcpiError::AE_CODE_CONTROL,
    ControlTerminate = 0x0003 | AcpiError::AE_CODE_CONTROL,
    ControlTrue = 0x0004 | AcpiError::AE_CODE_CONTROL,
    ControlFalse = 0x0005 | AcpiError::AE_CODE_CONTROL,
    ControlDepth = 0x0006 | AcpiError::AE_CODE_CONTROL,
    ControlEnd = 0x0007 | AcpiError::AE_CODE_CONTROL,
    ControlTransfer = 0x0008 | AcpiError::AE_CODE_CONTROL,
    ControlBreak = 0x0009 | AcpiError::AE_CODE_CONTROL,
    ControlContinue = 0x000A | AcpiError::AE_CODE_CONTROL,
    ControlParseContinue = 0x000B | AcpiError::AE_CODE_CONTROL,
    ControlParsePending = 0x000C | AcpiError::AE_CODE_CONTROL,

    Unknown,
}

impl AcpiError {
    const AE_CODE_ENVIRONMENTAL: u32 = 0x0000;
    const AE_CODE_PROGRAMMER: u32 = 0x1000;
    const AE_CODE_ACPI_TABLES: u32 = 0x2000;
    const AE_CODE_AML: u32 = 0x3000;
    const AE_CODE_CONTROL: u32 = 0x4000;
    const AE_CODE_MASK: u32 = 0xF000;
}

impl AcpiError {
    pub const fn is_env_exception(&self) -> bool {
        *self as u32 & Self::AE_CODE_MASK == Self::AE_CODE_ENVIRONMENTAL
    }
    pub const fn is_aml_exception(&self) -> bool {
        *self as u32 & Self::AE_CODE_MASK == Self::AE_CODE_AML
    }
    pub const fn is_programmer_exception(&self) -> bool {
        *self as u32 & Self::AE_CODE_MASK == Self::AE_CODE_PROGRAMMER
    }
    pub const fn is_table_exception(&self) -> bool {
        *self as u32 & Self::AE_CODE_MASK == Self::AE_CODE_ACPI_TABLES
    }
    pub const fn is_control_exception(&self) -> bool {
        *self as u32 & Self::AE_CODE_MASK == Self::AE_CODE_CONTROL
    }
}

pub(crate) trait AcpiErrorAsStatusExt {
    fn as_acpi_status(&self) -> AcpiStatus;
}

impl AcpiErrorAsStatusExt for Result<(), AcpiError> {
    fn as_acpi_status(&self) -> AcpiStatus {
        match self {
            Ok(_) => AcpiStatus(0),
            Err(e) => e.as_acpi_status(),
        }
    }
}

impl AcpiErrorAsStatusExt for AcpiError {
    fn as_acpi_status(&self) -> AcpiStatus {
        if let AcpiError::Unknown = self {
            AcpiStatus(Self::Error as u32)
        } else {
            AcpiStatus(*self as u32)
        }
    }
}

impl AcpiStatus {
    pub const OK: Self = Self(0);

    pub fn as_result(&self) -> Result<(), AcpiError> {
        match self.0 {
            0 => Ok(()),
            e if e == 0x0001 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::Error),
            e if e == 0x0002 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NoAcpiTables),
            e if e == 0x0003 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NoNamespace),
            e if e == 0x0004 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NoMemory),
            e if e == 0x0005 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NotFound),
            e if e == 0x0006 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NotExist),
            e if e == 0x0007 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::AlreadyExists),
            e if e == 0x0008 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::Type),
            e if e == 0x0009 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NullObject),
            e if e == 0x000A | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NullEntry),
            e if e == 0x000B | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::BufferOverflow),
            e if e == 0x000C | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::StackOverflow),
            e if e == 0x000D | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::StackUnderflow),
            e if e == 0x000E | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NotImplemented),
            e if e == 0x000F | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::Support),
            e if e == 0x0010 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::Limit),
            e if e == 0x0011 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::Time),
            e if e == 0x0012 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::AcquireDeadlock),
            e if e == 0x0013 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::ReleaseDeadlock),
            e if e == 0x0014 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NotAcquired),
            e if e == 0x0015 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::AlreadyAcquired),
            e if e == 0x0016 | AcpiError::AE_CODE_ENVIRONMENTAL => {
                Err(AcpiError::NoHardwareResponse)
            }
            e if e == 0x0017 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NoGlobalLock),
            e if e == 0x0018 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::AbortMethod),
            e if e == 0x0019 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::SameHandler),
            e if e == 0x001A | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NoHandler),
            e if e == 0x001B | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::OwnerIdLimit),
            e if e == 0x001C | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NotConfigured),
            e if e == 0x001D | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::Access),
            e if e == 0x001E | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::IoError),
            e if e == 0x001F | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::NumericOverflow),
            e if e == 0x0020 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::HexOverflow),
            e if e == 0x0021 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::DecimalOverflow),
            e if e == 0x0022 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::OctalOverflow),
            e if e == 0x0023 | AcpiError::AE_CODE_ENVIRONMENTAL => Err(AcpiError::EndOfTable),
            e if e == 0x0001 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadParameter),
            e if e == 0x0002 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadCharacter),
            e if e == 0x0003 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadPathname),
            e if e == 0x0004 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadData),
            e if e == 0x0005 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadHexConstant),
            e if e == 0x0006 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadOctalConstant),
            e if e == 0x0007 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadDecimalConstant),
            e if e == 0x0008 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::MissingArguments),
            e if e == 0x0009 | AcpiError::AE_CODE_PROGRAMMER => Err(AcpiError::BadAddress),
            e if e == 0x0001 | AcpiError::AE_CODE_ACPI_TABLES => Err(AcpiError::BadSignature),
            e if e == 0x0002 | AcpiError::AE_CODE_ACPI_TABLES => Err(AcpiError::BadHeader),
            e if e == 0x0003 | AcpiError::AE_CODE_ACPI_TABLES => Err(AcpiError::BadChecksum),
            e if e == 0x0004 | AcpiError::AE_CODE_ACPI_TABLES => Err(AcpiError::BadValue),
            e if e == 0x0005 | AcpiError::AE_CODE_ACPI_TABLES => Err(AcpiError::InvalidTableLength),
            e if e == 0x0001 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlBadOpcode),
            e if e == 0x0002 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNoOperand),
            e if e == 0x0003 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlOperandType),
            e if e == 0x0004 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlOperandValue),
            e if e == 0x0005 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlUninitializedLocal),
            e if e == 0x0006 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlUninitializedArg),
            e if e == 0x0007 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlUninitializedElement),
            e if e == 0x0008 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNumericOverflow),
            e if e == 0x0009 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlRegionLimit),
            e if e == 0x000A | AcpiError::AE_CODE_AML => Err(AcpiError::AmlBufferLimit),
            e if e == 0x000B | AcpiError::AE_CODE_AML => Err(AcpiError::AmlPackageLimit),
            e if e == 0x000C | AcpiError::AE_CODE_AML => Err(AcpiError::AmlDivideByZero),
            e if e == 0x000D | AcpiError::AE_CODE_AML => Err(AcpiError::AmlBadName),
            e if e == 0x000E | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNameNotFound),
            e if e == 0x000F | AcpiError::AE_CODE_AML => Err(AcpiError::AmlInternal),
            e if e == 0x0010 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlInvalidSpaceId),
            e if e == 0x0011 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlStringLimit),
            e if e == 0x0012 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNoReturnValue),
            e if e == 0x0013 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlMethodLimit),
            e if e == 0x0014 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNotOwner),
            e if e == 0x0015 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlMutexOrder),
            e if e == 0x0016 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlMutexNotAcquired),
            e if e == 0x0017 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlInvalidResourceType),
            e if e == 0x0018 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlInvalidIndex),
            e if e == 0x0019 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlRegisterLimit),
            e if e == 0x001A | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNoWhile),
            e if e == 0x001B | AcpiError::AE_CODE_AML => Err(AcpiError::AmlAlignment),
            e if e == 0x001C | AcpiError::AE_CODE_AML => Err(AcpiError::AmlNoResourceEndTag),
            e if e == 0x001D | AcpiError::AE_CODE_AML => Err(AcpiError::AmlBadResourceValue),
            e if e == 0x001E | AcpiError::AE_CODE_AML => Err(AcpiError::AmlCircularReference),
            e if e == 0x001F | AcpiError::AE_CODE_AML => Err(AcpiError::AmlBadResourceLength),
            e if e == 0x0020 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlIllegalAddress),
            e if e == 0x0021 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlLoopTimeout),
            e if e == 0x0022 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlUninitializedNode),
            e if e == 0x0023 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlTargetType),
            e if e == 0x0024 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlProtocol),
            e if e == 0x0025 | AcpiError::AE_CODE_AML => Err(AcpiError::AmlBufferLength),
            e if e == 0x0001 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlReturnValue),
            e if e == 0x0002 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlPending),
            e if e == 0x0003 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlTerminate),
            e if e == 0x0004 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlTrue),
            e if e == 0x0005 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlFalse),
            e if e == 0x0006 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlDepth),
            e if e == 0x0007 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlEnd),
            e if e == 0x0008 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlTransfer),
            e if e == 0x0009 | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlBreak),
            e if e == 0x000A | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlContinue),
            e if e == 0x000B | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlParseContinue),
            e if e == 0x000C | AcpiError::AE_CODE_CONTROL => Err(AcpiError::ControlParsePending),

            _ => Err(AcpiError::Unknown),
        }
    }
}

impl Display for AcpiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let err_string = match self {
            Self::Error => "Unspecified error",
            Self::NoAcpiTables => "ACPI tables could not be found",
            Self::NoNamespace => "A namespace has not been loaded",
            Self::NoMemory => "Insufficient dynamic memory",
            Self::NotFound => "A requested entity is not found",
            Self::NotExist => "A required entity does not exist",
            Self::AlreadyExists => "An entity already exists",
            Self::Type => "The object type is incorrect",
            Self::NullObject => "A required object was missing",
            Self::NullEntry => "The requested object does not exist",
            Self::BufferOverflow => "The buffer provided is too small",
            Self::StackOverflow => "An internal stack overflowed",
            Self::StackUnderflow => "An internal stack underflowed",
            Self::NotImplemented => "The feature is not implemented",
            Self::Support => "The feature is not supported",
            Self::Limit => "A predefined limit was exceeded",
            Self::Time => "A time limit or timeout expired",
            Self::AcquireDeadlock => {
                "Internal error, attempt was made to acquire a mutex in improper order"
            }
            Self::ReleaseDeadlock => {
                "Internal error, attempt was made to release a mutex in improper order"
            }
            Self::NotAcquired => {
                "An attempt to release a mutex or Global Lock without a previous acquire"
            }
            Self::AlreadyAcquired => "Internal error, attempt was made to acquire a mutex twice",
            Self::NoHardwareResponse => "Hardware did not respond after an I/O operation",
            Self::NoGlobalLock => "There is no FACS Global Lock",
            Self::AbortMethod => "A control method was aborted",
            Self::SameHandler => {
                "Attempt was made to install the same handler that is already installed"
            }
            Self::NoHandler => "A handler for the operation is not installed",
            Self::OwnerIdLimit => {
                "There are no more Owner IDs available for ACPI tables or control methods"
            }
            Self::NotConfigured => {
                "The interface is not part of the current subsystem configuration"
            }
            Self::Access => "Permission denied for the requested operation",
            Self::IoError => "An I/O error occurred",
            Self::NumericOverflow => "Overflow during string-to-integer conversion",
            Self::HexOverflow => "Overflow during ASCII hex-to-binary conversion",
            Self::DecimalOverflow => "Overflow during ASCII decimal-to-binary conversion",
            Self::OctalOverflow => "Overflow during ASCII octal-to-binary conversion",
            Self::EndOfTable => "Reached the end of table",
            Self::BadParameter => "A parameter is out of range or invalid",
            Self::BadCharacter => "An invalid character was found in a name",
            Self::BadPathname => "An invalid character was found in a pathname",
            Self::BadData => "A package or buffer contained incorrect data",
            Self::BadHexConstant => "Invalid character in a Hex constant",
            Self::BadOctalConstant => "Invalid character in an Octal constant",
            Self::BadDecimalConstant => "Invalid character in a Decimal constant",
            Self::MissingArguments => "Too few arguments were passed to a control method",
            Self::BadAddress => "An illegal null I/O address",
            Self::BadSignature => "An ACPI table has an invalid signature",
            Self::BadHeader => "Invalid field in an ACPI table header",
            Self::BadChecksum => "An ACPI table checksum is not correct",
            Self::BadValue => "An invalid value was found in a table",
            Self::InvalidTableLength => "The FADT or FACS has improper length",
            Self::AmlBadOpcode => "Invalid AML opcode encountered",
            Self::AmlNoOperand => "A required operand is missing",
            Self::AmlOperandType => "An operand of an incorrect type was encountered",
            Self::AmlOperandValue => "The operand had an inappropriate or invalid value",
            Self::AmlUninitializedLocal => "Method tried to use an uninitialized local variable",
            Self::AmlUninitializedArg => "Method tried to use an uninitialized argument",
            Self::AmlUninitializedElement => "Method tried to use an empty package element",
            Self::AmlNumericOverflow => "Overflow during BCD conversion or other",
            Self::AmlRegionLimit => "Tried to access beyond the end of an Operation Region",
            Self::AmlBufferLimit => "Tried to access beyond the end of a buffer",
            Self::AmlPackageLimit => "Tried to access beyond the end of a package",
            Self::AmlDivideByZero => "During execution of AML Divide operator",
            Self::AmlBadName => "An ACPI name contains invalid character(s)",
            Self::AmlNameNotFound => "Could not resolve a named reference",
            Self::AmlInternal => "An internal error within the interpreter",
            Self::AmlInvalidSpaceId => "An Operation Region SpaceID is invalid",
            Self::AmlStringLimit => "String is longer than 200 characters",
            Self::AmlNoReturnValue => "A method did not return a required value",
            Self::AmlMethodLimit => "A control method reached the maximum reentrancy limit of 255",
            Self::AmlNotOwner => "A thread tried to release a mutex that it does not own",
            Self::AmlMutexOrder => "Mutex SyncLevel release mismatch",
            Self::AmlMutexNotAcquired => {
                "Attempt to release a mutex that was not previously acquired"
            }
            Self::AmlInvalidResourceType => "Invalid resource type in resource list",
            Self::AmlInvalidIndex => "Invalid Argx or Localx (x too large)",
            Self::AmlRegisterLimit => "Bank value or Index value beyond range of register",
            Self::AmlNoWhile => "Break or Continue without a While",
            Self::AmlAlignment => {
                "Non-aligned memory transfer on platform that does not support this"
            }
            Self::AmlNoResourceEndTag => "No End Tag in a resource list",
            Self::AmlBadResourceValue => "Invalid value of a resource element",
            Self::AmlCircularReference => "Two references refer to each other",
            Self::AmlBadResourceLength => {
                "The length of a Resource Descriptor in the AML is incorrect"
            }
            Self::AmlIllegalAddress => "A memory, I/O, or PCI configuration address is invalid",
            Self::AmlLoopTimeout => "An AML While loop exceeded the maximum execution time",
            Self::AmlUninitializedNode => "A namespace node is uninitialized or unresolved",
            Self::AmlTargetType => "A target operand of an incorrect type was encountered",
            Self::AmlProtocol => "Violation of a fixed ACPI protocol",
            Self::AmlBufferLength => "The length of the buffer is invalid/incorrect",
            Self::ControlReturnValue => "A Method returned a value",
            Self::ControlPending => "Method is calling another method",
            Self::ControlTerminate => "Terminate the executing method",
            Self::ControlTrue => "An If or While predicate result",
            Self::ControlFalse => "An If or While predicate result",
            Self::ControlDepth => "Maximum search depth has been reached",
            Self::ControlEnd => "An If or While predicate is false",
            Self::ControlTransfer => "Transfer control to called method",
            Self::ControlBreak => "A Break has been executed",
            Self::ControlContinue => "A Continue has been executed",
            Self::ControlParseContinue => "Used to skip over bad opcodes",
            Self::ControlParsePending => "Used to implement AML While loops",

            Self::Unknown => "Unknown error",
        };

        f.write_str(err_string)
    }
}

/// Tests that [`as_result`][AcpiStatus::as_result] is the inverse of [`as_acpi_status`][AcpiError::as_acpi_status]
#[test]
fn test_as_result() {
    assert_eq!(AcpiStatus(0).as_result(), Ok(()));

    for i in 1..0x4f00 {
        let error = AcpiStatus(i)
            .as_result()
            .expect_err("An AcpiStatus with a non-0 value should not map to Ok");
        
        if error == AcpiError::Unknown {
            continue;
        }

        assert_eq!(error.as_acpi_status().0, i);
    }
}
