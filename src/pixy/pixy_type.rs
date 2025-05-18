use ufmt::uDisplay;

#[non_exhaustive]
#[repr(u8)]
/// A Packet Type for PixyCam
pub enum PacketType {
    RequestResolution = 0x0c,
    ResponseResolution = 0x0d,

    RequestVersion = 0x0e,
    ResponseVersion = 0x0f,

    ResponseResult = 0x01,
    ResponseError = 0x03,

    RequestBlocks = 0x20,
    ResponseBlocks = 0x21,

    Invalid,
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0x0c => Self::RequestResolution,
            0x0d => Self::ResponseResolution,
            0x0e => Self::RequestVersion,
            0x0f => Self::ResponseVersion,
            0x01 => Self::ResponseResult,
            0x03 => Self::ResponseError,
            0x20 => Self::RequestBlocks,
            0x21 => Self::ResponseBlocks,

            _ => Self::Invalid,
        }
    }
}

/// A result type for PixyCam
#[repr(i8)]
pub enum PixyResultType {
    Ok = 0,
    Error = -1,
    Busy = -2,
    ChecksumError = -3,
    Timeout = -4,
    ButtonOverride = -5,
    ProgramChanging = -6,

    Invalid = -69,
}

impl uDisplay for PixyResultType {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            PixyResultType::Ok => f.write_str("Ok"),
            PixyResultType::Error => f.write_str("Error"),
            PixyResultType::Busy => f.write_str("Busy"),
            PixyResultType::ChecksumError => f.write_str("Checksum Error"),
            PixyResultType::Timeout => f.write_str("Timeout"),
            PixyResultType::ButtonOverride => f.write_str("Button Override"),
            PixyResultType::ProgramChanging => f.write_str("Program Changing"),
            PixyResultType::Invalid => f.write_str("Invalid Error Type"),
        }
    }
}

impl From<i8> for PixyResultType {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Ok,
            -1 => Self::Error,
            -2 => Self::Busy,
            -3 => Self::ChecksumError,
            -4 => Self::Timeout,
            -5 => Self::ButtonOverride,
            -6 => Self::ProgramChanging,

            _ => Self::Invalid,
        }
    }
}
