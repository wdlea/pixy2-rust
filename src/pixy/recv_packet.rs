use embedded_hal::{delay::DelayNs, spi::SpiDevice};
use ufmt::{uDebug, uwriteln};



use super::{Pixy2, get_sync::SyncError};

pub enum RecvError<Link: SpiDevice> {
    SyncError(SyncError<Link>),
    ReadError(Link::Error),
    InvalidChecksum,
}

impl<Link: SpiDevice> uDebug for RecvError<Link> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            RecvError::SyncError(sync_error) => {
                uwriteln!(f, "Failed to get sync: {:?}", sync_error)
            }

            RecvError::ReadError(_) => uwriteln!(f, "Failed to read"),
            RecvError::InvalidChecksum => uwriteln!(f, "Invalid Checksum"),
        }
    }
}

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
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
