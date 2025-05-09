#![no_std]
#![deny(missing_docs, clippy::empty_docs)]

//! A port of the (Pixy2 Library)[github.com/charmedlabs/pixy2/](Originally C++) to Rust.
//! Originally licensed under the (GNU General Public License v2)[www.gnu.org/licenses/gpl-2.0.html].

/// An abstraction over a communication protocol.
pub mod link_type;
/// The core functionality of this package.
pub mod pixy;

/// The [version::Version] type, which represents a PixyCam Version
pub mod version;
