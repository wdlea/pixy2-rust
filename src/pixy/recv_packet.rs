use core::fmt::Debug;

use crate::link_type::LinkType;

use super::{Pixy2, get_sync::SyncError};

pub enum RecvError<Link: LinkType> {
    SyncError(SyncError<Link>),
    ReadError(Link::ReadError),
    InvalidChecksum,
}

impl<Link: LinkType> Debug for RecvError<Link> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SyncError(arg0) => f.debug_tuple("SyncError").field(arg0).finish(),
            Self::ReadError(arg0) => f.debug_tuple("ReadError").field(arg0).finish(),
            Self::InvalidChecksum => write!(f, "InvalidChecksum"),
        }
    }
}

impl<Link: LinkType> Pixy2<Link> {
    /// Receive the next packet that the camera sends
    pub fn recv_packet(&mut self) -> Result<(u8, &mut [u8]), RecvError<Link>> {
        self.get_sync().map_err(|e| RecvError::SyncError(e))?;

        self.link
            .read(&mut self.buf[0..2])
            .map_err(|e| RecvError::ReadError(e))?;

        let message_type = self.buf[0].to_le(); // this will convert from le to native endianness as it flips the order 
        let message_length = self.buf[1].to_le();

        let buf;

        if self.using_checksums {
            self.link
                .read(&mut self.buf[0..2])
                .map_err(|e| RecvError::ReadError(e))?;

            let message_checksum: u16 = unsafe { *self.buf[0..2].as_ptr().cast::<u16>() }.to_le();

            buf = &mut self.buf[..(message_length as usize)];

            self.link.read(buf).map_err(|e| RecvError::ReadError(e))?;

            let checksum_calculation: u16 = buf.iter().map(|i| i.to_le() as u16).sum();

            if message_checksum != checksum_calculation {
                return Err(RecvError::InvalidChecksum);
            }
        } else {
            buf = &mut self.buf[..(message_length as usize)];

            self.link.read(buf).map_err(|e| RecvError::ReadError(e))?;
        }

        buf.iter_mut().for_each(|i| *i = i.to_le());

        Ok((message_type, buf))
    }
}
