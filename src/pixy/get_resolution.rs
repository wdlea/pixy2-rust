use crate::link_type::LinkType;

use super::{Pixy2, operation_error::OperationError};

const REQUEST_PIXY_RESOLOUTION: u8 = 0x0c;
const RESPONSE_PIXY_RESOLOUTION: u8 = 0x0d;

impl<Link: LinkType> Pixy2<Link> {
    pub fn get_resolution(&mut self) -> Result<(u16, u16), OperationError<Link>> {
        self.send_packet(REQUEST_PIXY_RESOLOUTION, &[0])
            .map_err(|e| OperationError::SendError(e))?;

        let (resp_type, resp_payload) = self
            .recv_packet()
            .map_err(|e| OperationError::RecvError(e))?;

        if resp_type != RESPONSE_PIXY_RESOLOUTION {
            return Err(OperationError::UnexpectedPacket {
                got: resp_type,
                expected: RESPONSE_PIXY_RESOLOUTION,
            });
        }

        let (width, height): (u16, u16) = unsafe { *resp_payload.as_ptr().cast::<(u16, u16)>() };

        self.frame_width = Some(width);
        self.frame_height = Some(height);

        Ok((width, height))
    }
}
