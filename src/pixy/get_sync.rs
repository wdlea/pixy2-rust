use embedded_hal::delay::DelayNs;
use ufmt::{uDebug, uwriteln};

use crate::link_type::LinkType;

use super::Pixy2;

const PIXY_CHECKSUM_SYNC: u16 = 0xc1af_u16;
pub const PIXY_NO_CHECKSUM_SYNC: u16 = 0xc1ae_u16;

pub enum SyncError<Link: LinkType> {
    NoSync,
    ReadError(Link::ReadError),
    Other(u8),
}

impl<Link: LinkType> uDebug for SyncError<Link> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            SyncError::NoSync => uwriteln!(f, "No Sync Found"),
            SyncError::ReadError(_) => uwriteln!(f, "Could not read for sync."),
            SyncError::Other(msg) => uwriteln!(f, "Other Error: {}", msg),
        }
    }
}

impl<Link: LinkType, W: DelayNs> Pixy2<Link, W> {
    /// Waits until a sync sequence is received from the camera.
    pub fn get_sync(&mut self) -> Result<(), SyncError<Link>> {
        let [mut i, mut j, mut cprev] = [0u8; 3];
        let mut start: u16;

        // i = 0, and j = 0
        loop {
            let buf = &mut self.buf[0..1];
            self.link.read(buf).map_err(|e| SyncError::ReadError(e))?;
            let c = buf[0];

            start = cprev as u16; // assuming little-endian system
            start |= (c as u16) << 8;
            cprev = c;

            if start == PIXY_CHECKSUM_SYNC {
                self.using_checksums = true;
                return Ok(());
            } else if start == PIXY_NO_CHECKSUM_SYNC {
                self.using_checksums = false;
                return Ok(());
            }

            if i >= 4 {
                if j >= 4 {
                    return Err(SyncError::NoSync);
                }
                j += 1;
                i = 0;

                self.waiter.delay_us(25);
            }

            i += 1;
        }
    }
}
