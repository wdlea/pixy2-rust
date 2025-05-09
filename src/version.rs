use core::str;

use ufmt::{uDisplay, uwrite};

#[repr(C)]
#[derive(Clone)]
/// Represents a PixyCam Version
pub struct Version {
    /// The hardware version
    pub hardware: u16,
    /// The firmware major version
    pub firmware_major: u8,
    /// The firmware minor version
    pub firmware_minor: u8,
    /// The firmware build version
    pub firmware_build: u16,

    /// The firmware type
    pub firmware_type: [u8; 10],
}

impl uDisplay for Version {
    fn fmt<W>(&self, w: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        uwrite!(
            w,
            "Hardware Version: {}, Firmware Version: {}.{}.{} {}",
            self.hardware,
            self.firmware_major,
            self.firmware_minor,
            self.firmware_build,
            str::from_utf8(self.firmware_type.as_slice()).unwrap()
        )
    }
}
