use crate::{link_type::LinkType, version::Version};

use super::{Pixy2, operation_error::OperationError};

pub const REQUEST_PIXY_VERSION: u8 = 0x0e;
pub const RESPONSE_PIXY_VERSION: u8 = 0x0f;

impl<Link: LinkType> Pixy2<Link> {
    /// Requests the camera's version.
    pub fn get_version(&mut self) -> Result<&Version, OperationError<Link>> {
        self.send_packet(REQUEST_PIXY_VERSION, &[])
            .map_err(|e| OperationError::<Link>::SendError(e))?;

        let (resp_type, resp_payload) = self
            .recv_packet()
            .map_err(|e| OperationError::RecvError(e))?;

        if resp_type != RESPONSE_PIXY_VERSION {
            return Err(OperationError::UnexpectedPacket {
                got: resp_type,
                expected: REQUEST_PIXY_VERSION,
            });
        }

        let version: Version = unsafe {
            (resp_payload.as_ptr())
                .cast::<Version>()
                .as_ref()
                .unwrap()
                .clone()
        };

        self.version = Some(version);
        Ok(self.version.as_ref().unwrap())
    }
}
