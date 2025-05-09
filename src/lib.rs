#![no_std]
#![deny(missing_docs, clippy::empty_docs, clippy::missing_docs_in_private_items)]

//! A port of the (Pixy2 Library)[https://github.com/charmedlabs/pixy2/](Originally C++) to Rust.
//! Originally licensed under the (GNU General Public License v2)[http://www.gnu.org/licenses/gpl-2.0.html].

/// An abstraction over a communication protocol.
pub mod link_type;
/// The core functionality of this package.
pub mod pixy;

/// The [Version] type, which represents a PixyCam Version
pub mod version;
