use embedded_hal::{delay::DelayNs, spi::SpiDevice};
use ufmt::{uWrite, uwriteln};

use crate::version::Version;

mod get_blocks;
mod get_resolution;
mod get_sync;
mod get_version;
mod operation_error;
mod recv_packet;
mod send_packet;

pub use get_blocks::Block;
pub use operation_error::OperationError;

/// Represents a Pixy2 camera
pub struct Pixy2<Link, W> {
    /// The version of this camera, if it has been fetched
    pub version: Option<Version>,
    /// The width (in pixels) of the camera
    pub frame_width: Option<u16>,
    /// The height (in pixels) of the camera
    pub frame_height: Option<u16>,

    link: Link,
    using_checksums: bool,
    buf: [u8; 256],
    waiter: W,
}

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
    /// Create an initialize a Pixy2 object.
    pub fn new(link: Link, waiter: W, dbg: &mut impl uWrite) -> Result<Self, OperationError<Link>> {
        let mut me = Self {
            version: None,
            frame_width: None,
            frame_height: None,
            link,
            using_checksums: false,
            buf: [0; 256],
            waiter,
        };
        let mut i = 0;

        while i < 1000 {
            // delay for at most 5_000_000 us (5 seconds)

            match me.get_version() {
                Ok(_) => {
                    me.get_resolution()?;
                    return Ok(me);
                }
                Err(e) => {
                    _ = uwriteln!(dbg, "{:?}", e);
                }
            }

            me.waiter.delay_us(5_000);
            i += 1;
        }

        Err(OperationError::Timeout)
    }
}
