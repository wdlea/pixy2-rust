use embedded_hal::delay::DelayNs;
use embedded_time::{Clock, duration::Milliseconds};
use operation_error::OperationError;

use crate::{link_type::LinkType, version::Version};

mod get_resolution;
mod get_sync;
mod get_version;
mod operation_error;
mod recv_packet;
mod send_packet;

pub struct Pixy2<Link> {
    pub version: Option<Version>,
    pub frame_width: Option<u16>,
    pub frame_height: Option<u16>,

    link: Link,
    using_checksums: bool,
    buf: [u8; 256],
}

impl<Link: LinkType> Pixy2<Link> {
    pub fn new(
        link: Link,
        clock: impl Clock,
        waiter: &mut impl DelayNs,
    ) -> Result<Self, OperationError<Link>> {
        let mut me = Self {
            version: None,
            frame_width: None,
            frame_height: None,
            link,
            using_checksums: false,
            buf: [0; 256],
        };
        let start_time = clock.try_now().map_err(|e| OperationError::ClockError(e))?;
        let end_time = start_time + Milliseconds(5_000);

        while clock.try_now().map_err(|e| OperationError::ClockError(e))? < end_time {
            if me.get_version().is_ok() {
                me.get_resolution()?;
                return Ok(me);
            }
            waiter.delay_us(5_000);
        }

        Err(OperationError::Timeout)
    }
}
