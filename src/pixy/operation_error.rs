use embedded_io::SliceWriteError;
use embedded_time::clock;
use ufmt::{uDebug, uwriteln};

use crate::link_type::LinkType;

use super::recv_packet::RecvError;

/// Errors which can arise from any operation]
pub enum OperationError<Link: LinkType> {
    /// An error raised while sending a packet
    SendError(Link::WriteError),
    /// An error raised when receiving a packet
    RecvError(RecvError<Link>),

    /// An unexpected packet was received
    UnexpectedPacket {
        /// What the type of the received packet was
        got: u8,
        /// What was expected. There may be more valid packets to receive.
        expected: u8,
    },
    /// An error occurred in [embedded_time::clock]
    ClockError(clock::Error),
    /// An error occurred in [embedded_io]
    IOError(SliceWriteError),

    /// An error occurred from the Pixy2
    PixyError(i8),

    /// The Pixy2 camera could not be reached
    Timeout,
    /// The Pixy2 camera has bigger fish to fry
    Busy,
}

impl<Link: LinkType> uDebug for OperationError<Link> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            OperationError::SendError(_) => uwriteln!(f, "Send Error"),
            OperationError::RecvError(err) => uwriteln!(f, "Receive Error: {:?}", err),
            OperationError::UnexpectedPacket {
                got: _,
                expected: _,
            } => f.write_str("Unexpected Packet"),
            OperationError::ClockError(_) => f.write_str("Clock Error"),
            OperationError::IOError(_) => f.write_str("IO Error"),
            OperationError::PixyError(_) => f.write_str("Pixy Error"),
            OperationError::Timeout => f.write_str("Timeout"),
            OperationError::Busy => f.write_str("Pixy is Busy"),
        }
    }
}
