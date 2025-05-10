use core::fmt::Debug;

use embedded_hal::spi::{ErrorType, SpiDevice};

/// Something that can be used to communicate with Pixy2
pub trait LinkType {
    /// An error type that may be thrown by read
    type ReadError: Debug;
    /// An error type that may be thrown by write
    type WriteError: Debug;

    /// Write bytes to the Pixy2 camera
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::WriteError>;

    /// Read bytes from Pixy2 camera to completely full up buffer
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::ReadError>;
}

impl<T: SpiDevice + ErrorType> LinkType for T {
    type ReadError = T::Error;
    type WriteError = T::Error;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::WriteError> {
        self.write(buf)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::ReadError> {
        self.read(buf)
    }
}
