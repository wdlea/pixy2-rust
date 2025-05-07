use embedded_io::{Read, ReadReady, Write};

pub trait LinkType: Write + Read + ReadReady {}
