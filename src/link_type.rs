use embedded_io::{Read, Write};

pub trait LinkType: Write + Read {}
