use embedded_hal::{delay::DelayNs, spi::SpiDevice};

use super::{Pixy2, operation_error::OperationError, pixy_type::PacketType};

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
    /// Returns the resolution of the camera in pixels.
    pub fn get_resolution(&mut self) -> Result<(u16, u16), OperationError<Link>> {
        self.send_packet(PacketType::RequestResolution, &[0])
            .map_err(|e| OperationError::SendError(e))?;

        let (resp_type, resp_payload) = self
            .recv_packet()
            .map_err(|e| OperationError::RecvError(e))?;

        if !matches!(resp_type, PacketType::ResponseResolution) {
            return Err(OperationError::UnexpectedPacket {
                got: resp_type,
                expected: PacketType::RequestResolution,
            });
        }

        let (width, height): (u16, u16) = unsafe { *resp_payload.as_ptr().cast::<(u16, u16)>() };

        self.frame_width = Some(width);
        self.frame_height = Some(height);

        Ok((width, height))
    }
}
