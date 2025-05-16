use core::fmt::Debug;

use ufmt::{uDebug, uwriteln};

use crate::link_type::LinkType;

use super::Pixy2;

const PIXY_CHECKSUM_SYNC: u16 = 0xc1af_u16;
pub const PIXY_NO_CHECKSUM_SYNC: u16 = 0xc1ae_u16;

pub enum SyncError<Link: LinkType> {
    NoSync,
    ReadError(Link::ReadError),
}

impl<Link: LinkType> uDebug for SyncError<Link> {
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

impl<Link: LinkType> Debug for SyncError<Link> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoSync => write!(f, "NoSync"),
            Self::ReadError(arg0) => f.debug_tuple("ReadError").field(arg0).finish(),
        }
    }
}

impl<Link: LinkType> Pixy2<Link> {
    /// Waits until a sync sequence is received from the camera.
    pub fn get_sync(&mut self) -> Result<(), SyncError<Link>> {
        let mut prev = 0u8;
        let mut i = 0;
        loop {
            let mut buf = [0u8];

            self.link
                .read(buf.as_mut_slice())
                .map_err(|e| SyncError::ReadError(e))?;

            let current = buf[0] as u16;

            let start: u16 = (current) | ((prev as u16) << 8);
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
