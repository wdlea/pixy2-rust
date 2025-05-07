use crate::version::Version;

mod get_sync;

pub struct Pixy2<Link> {
    pub version: Version,
    pub frame_width: u16,
    pub frame_height: u16,

    pub link: Link,

    using_checksums: bool,
}
