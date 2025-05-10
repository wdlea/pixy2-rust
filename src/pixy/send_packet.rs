use embedded_io::{Read, ReadReady, Write};

use super::{Pixy2, get_sync::PIXY_NO_CHECKSUM_SYNC};

impl<Link: Write + Read + ReadReady> Pixy2<Link> {
    /// Send a packed with a type and a payload.
    /// This uses the length of the slice for message
    /// length.
    pub fn send_packet(&mut self, packet_type: u8, payload: &[u8]) -> Result<(), Link::Error> {
        self.link.write_all(&PIXY_NO_CHECKSUM_SYNC.to_le_bytes())?;
        self.link.write_all(&packet_type.to_le_bytes())?;
        self.link.write_all(&(payload.len() as u8).to_le_bytes())?;
        self.link.write_all(payload)?;

        Ok(())
    }
}
