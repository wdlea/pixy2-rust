use crate::link_type::LinkType;

use super::{Pixy2, get_sync::PIXY_NO_CHECKSUM_SYNC};

impl<Link: LinkType> Pixy2<Link> {
    /// Send a packed with a type and a payload.
    /// This uses the length of the slice for message
    /// length.
    pub fn send_packet(&mut self, packet_type: u8, payload: &[u8]) -> Result<(), Link::WriteError> {
        self.link.write(&PIXY_NO_CHECKSUM_SYNC.to_le_bytes())?;
        self.link.write(&packet_type.to_le_bytes())?;
        self.link.write(&(payload.len() as u8).to_le_bytes())?;
        self.link.write(payload)?;

        Ok(())
    }
}
