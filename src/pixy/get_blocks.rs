use core::ptr::slice_from_raw_parts;

use embedded_hal::delay::DelayNs;
use embedded_io::{Read, ReadReady, Write};

use super::{Pixy2, operation_error::OperationError};

const REQUEST_BLOCKS: u8 = 0x20;
const RESPONSE_BLOCKS: u8 = 0x21;
const RESPONSE_ERROR: u8 = 0x03;
const RESPONSE_ERROR_BUSY: i8 = -2;
const RESPONSE_ERROR_PROG_CHANGING: i8 = -6;

/// Represents a region of colour tracked by PixyCam.
#[repr(C)]
pub struct Block {
    /// The signature of this block
    signature: u16,

    /// The x coordinate of this block on the camera.
    x: u16,
    /// The y coordinate of this block on the camera.
    y: u16,

    /// The width of this block (in pixels).
    width: u16,
    /// The height of this block (in pixels).
    height: u16,

    /// The angle of this block relative to the camera.
    angle: i16,
    /// The tracking index of this block.
    index: u8,
    /// How long has this been tracked for, in frames.
    /// Stops incrementing at 255.
    age: u8,
}

impl<Link: Write + Read + ReadReady> Pixy2<Link> {
    /// Get an array of blocks with the given `signature_bitmap`.
    /// It can either return immediately with [OperationError::Busy]
    /// if no blocks are found, or `wait`. Returns at most
    /// `max_blocks`.
    pub fn get_blocks(
        &mut self,
        wait: bool,
        signature_bitmap: u8,
        max_blocks: u8,
        waiter: &mut impl DelayNs,
    ) -> Result<&[Block], OperationError<Link>> {
        loop {
            self.send_packet(REQUEST_BLOCKS, &[signature_bitmap, max_blocks])
                .map_err(|e| OperationError::SendError(e))?;

            let (resp_type, resp_payload) = self
                .recv_packet()
                .map_err(|e| OperationError::RecvError(e))?;

            match resp_type {
                RESPONSE_BLOCKS => {
                    let slice = unsafe {
                        slice_from_raw_parts(
                            resp_payload.as_ptr().cast::<Block>(),
                            size_of_val(resp_payload) / size_of::<Block>(),
                        )
                        .as_ref()
                        .unwrap()
                    };

                    return Ok(slice);
                }
                RESPONSE_ERROR => {
                    let reinterpreted_i8 = unsafe { *(resp_payload[0] as *const i8) };

                    if reinterpreted_i8 == RESPONSE_ERROR_BUSY {
                        if wait {
                            continue;
                        } else {
                            return Err(OperationError::Busy);
                        }
                    } else if reinterpreted_i8 != RESPONSE_ERROR_PROG_CHANGING {
                        return Err(OperationError::PixyError(reinterpreted_i8));
                    }
                }
                _ => {
                    return Err(OperationError::UnexpectedPacket {
                        got: resp_type,
                        expected: RESPONSE_BLOCKS,
                    });
                }
            }

            waiter.delay_us(500);
        }
    }
}
