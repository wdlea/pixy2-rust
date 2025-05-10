use embedded_io::{Read, ReadReady, SliceWriteError, Write};
use embedded_time::clock;

use super::recv_packet::RecvError;

/// Errors which can arise from any operation
pub enum OperationError<Link: Read + Write + ReadReady> {
    /// An error raised while sending a packet
    SendError(Link::Error),
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
