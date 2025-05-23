use embedded_hal::{delay::DelayNs, spi::SpiDevice};

use super::{Pixy2, get_sync::PIXY_NO_CHECKSUM_SYNC, pixy_type::PacketType};

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
    /// Send a packed with a type and a payload.
    /// This uses the length of the slice for message
    /// length.
    pub fn send_packet(
        &mut self,
        packet_type: PacketType,
        payload: &[u8],
    ) -> Result<(), Link::Error> {
        self.link.write(&PIXY_NO_CHECKSUM_SYNC.to_le_bytes())?;
        self.link.write(&(packet_type as u8).to_le_bytes())?;
        self.link.write(&(payload.len() as u8).to_le_bytes())?;
        self.link.write(payload)?;

        Ok(())
    }
}
