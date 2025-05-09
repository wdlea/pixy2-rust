use embedded_io::SliceWriteError;
use embedded_time::clock;

use crate::link_type::LinkType;

use super::recv_packet::RecvError;

/// Errors which can arise from any operation
pub enum OperationError<Link: LinkType> {
    SendError(Link::Error),
    RecvError(RecvError<Link>),

    UnexpectedPacket { got: u8, expected: u8 },
    ClockError(clock::Error),
    IOError(SliceWriteError),

    PixyError(i8),

    Timeout,
    Busy,
}
