use embedded_hal::{delay::DelayNs, spi::SpiDevice};
use ufmt::{uDebug, uwriteln};

use super::Pixy2;

const PIXY_CHECKSUM_SYNC: u16 = 0xc1af_u16;
pub const PIXY_NO_CHECKSUM_SYNC: u16 = 0xc1ae_u16;

pub enum SyncError<Link: SpiDevice> {
    NoSync,
    ReadError(Link::Error),
}

impl<Link: SpiDevice> uDebug for SyncError<Link> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            SyncError::NoSync => uwriteln!(f, "No Sync Found"),
            SyncError::ReadError(_) => uwriteln!(f, "Could not read for sync."),
        }
    }
}

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
    /// Waits until a sync sequence is received from the camera.
    pub fn get_sync(&mut self) -> Result<(), SyncError<Link>> {
        let [mut i, mut j, mut previous] = [0u8; 3];
        let mut start: u16;

        // i = 0, and j = 0
        loop {
            let buf = &mut self.buf[0..1];
            self.link.read(buf).map_err(|e| SyncError::ReadError(e))?;
            let c = buf[0];

            start = previous as u16; // assuming little-endian system
            start |= (c as u16) << 8;
            previous = c;

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
