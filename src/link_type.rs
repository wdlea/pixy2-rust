use embedded_io::{Read, ReadReady, Write};

/// An abstraction over communication protocols that can be used with PixyCam
pub trait LinkType: Write + Read + ReadReady {}
