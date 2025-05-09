use embedded_time::{Clock, clock, duration::Milliseconds};

use crate::{link_type::LinkType, version::Version};

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
    pub fn new(link: Link, clock: impl Clock) -> Result<Self, clock::Error> {
        let mut me = Self {
            version: None,
            frame_width: None,
            frame_height: None,
            link,
            using_checksums: false,
            buf: [0; 256],
        };
        let start_time = clock.try_now()?;
        let end_time = start_time + Milliseconds(5_000);

        while clock.try_now()? < end_time {
            if me.get_version().is_ok() {
                todo!("Get res");
            }
        }

        todo!()
    }
}
