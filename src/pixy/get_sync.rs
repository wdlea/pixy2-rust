use embedded_io::ReadExactError;

use crate::link_type::LinkType;

use super::Pixy2;

const PIXY_CHECKSUM_SYNC: u16 = 0xc1af_u16;
pub const PIXY_NO_CHECKSUM_SYNC: u16 = 0xc1ae_u16;

pub enum SyncError<Link: LinkType> {
    NoSync,
    ReadError(ReadExactError<Link::Error>),
}

impl<Link: LinkType> Pixy2<Link> {
    pub fn get_sync(&mut self) -> Result<(), SyncError<Link>> {
        let mut prev = 0u8;
        let mut i = 0;
        loop {
            if self
                .link
                .read_ready()
                .map_err(|e| SyncError::ReadError(ReadExactError::Other(e)))?
            {
                let mut buf = [0u8];

                self.link
                    .read_exact(buf.as_mut_slice())
                    .map_err(|e| SyncError::ReadError(e))?;

                let current = buf[0] as u16;

                let start: u16 = (current << 8) | (prev as u16);
                if start == PIXY_CHECKSUM_SYNC {
                    self.using_checksums = false;
                    return Ok(());
                } else if start == PIXY_NO_CHECKSUM_SYNC {
                    self.using_checksums = true;
                    return Ok(());
                }

                prev = current as u8;

                if i > 128 {
                    return Err(SyncError::NoSync);
                }
                i += 1;
            }
        }
    }
}
