use embedded_hal::{delay::DelayNs, spi::SpiDevice};

use crate::version::Version;

use super::{Pixy2, operation_error::OperationError, pixy_type::PacketType};

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
    /// Requests the camera's version.
    pub fn get_version(&mut self) -> Result<&Version, OperationError<Link>> {
        self.send_packet(PacketType::RequestVersion, &[])
            .map_err(|e| OperationError::<Link>::SendError(e))?;

        let (resp_type, resp_payload) = self
            .recv_packet()
            .map_err(|e| OperationError::RecvError(e))?;

        if !matches!(resp_type, PacketType::ResponseVersion) {
            return Err(OperationError::UnexpectedPacket {
                got: resp_type,
                expected: PacketType::ResponseVersion,
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
