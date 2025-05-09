use std::ptr::slice_from_raw_parts;

use embedded_hal::delay::DelayNs;

use crate::link_type::LinkType;

use super::{Pixy2, operation_error::OperationError};

const REQUEST_BLOCKS: u8 = 0x20;
const RESPONSE_BLOCKS: u8 = 0x21;
const RESPONSE_ERROR: u8 = 0x03;
const RESPONSE_ERROR_BUSY: i8 = -2;
const RESPONSE_ERROR_PROG_CHANGING: i8 = -6;

#[repr(C)]
pub struct Block {
    signature: u16,

    x: u16,
    y: u16,

    width: u16,
    height: u16,

    angle: i16,
    index: u8,
    age: u8,
}

impl<Link: LinkType> Pixy2<Link> {
    pub fn get_blocks(
        &mut self,
        wait: bool,
        sigmap: u8,
        max_blocks: u8,
        waiter: &mut impl DelayNs,
    ) -> Result<&[Block], OperationError<Link>> {
        loop {
            self.send_packet(REQUEST_BLOCKS, &[sigmap, max_blocks])
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
