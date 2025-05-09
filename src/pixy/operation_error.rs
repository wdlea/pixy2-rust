use crate::link_type::LinkType;

use super::recv_packet::RecvError;

pub enum OperationError<Link: LinkType> {
    SendError(Link::Error),
    RecvError(RecvError<Link>),

    UnexpectedPacket { got: u8, expected: u8 },
}
