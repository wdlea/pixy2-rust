use embedded_io::ReadExactError;

use crate::link_type::LinkType;

use super::{Pixy2, get_sync::SyncError};

pub enum RecvError<Link: LinkType> {
    SyncError(SyncError<Link>),
    ReadError(ReadExactError<Link::Error>),
    InvalidChecksum,
}

impl<Link: LinkType> Pixy2<Link> {
    pub fn recv_packet(&mut self) -> Result<(u8, &mut [u8]), RecvError<Link>> {
        self.get_sync().map_err(|e| RecvError::SyncError(e))?;

        self.link
            .read_exact(&mut self.buf[0..2])
            .map_err(|e| RecvError::ReadError(e))?;

        let message_type = self.buf[0];
        let message_length = self.buf[1];

        let buf;

        if self.using_checksums {
            self.link
                .read_exact(&mut self.buf[0..2])
                .map_err(|e| RecvError::ReadError(e))?;

            let message_checksum: u16 = unsafe { *self.buf[0..2].as_ptr().cast() };

            buf = &mut self.buf[..(message_length as usize)];

            self.link
                .read_exact(buf)
                .map_err(|e| RecvError::ReadError(e))?;

            let checksum_calculation: u16 = buf.iter().map(|i| *i as u16).sum();

            if message_checksum != checksum_calculation {
                return Err(RecvError::InvalidChecksum);
            }
        } else {
            buf = &mut self.buf[..(message_length as usize)];

            self.link
                .read_exact(buf)
                .map_err(|e| RecvError::ReadError(e))?;
        }

        Ok((message_type, buf))
    }
}
