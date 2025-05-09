use std::fmt::Display;

#[repr(C)]
#[derive(Clone)]
pub struct Version {
    pub hardware: u16,
    pub firmware_major: u8,
    pub firmware_minor: u8,
    pub firmware_build: u16,

    pub firmware_type: [char; 10],
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hardware Version: {}, Firmware Version: {}.{}.{} {}",
            self.hardware,
            self.firmware_major,
            self.firmware_minor,
            self.firmware_build,
            String::from_iter(self.firmware_type.iter())
        )
    }
}
