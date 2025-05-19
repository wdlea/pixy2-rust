use super::{
    operation_error::OperationError,
    pixy_type::{PacketType, PixyResultType},
    Pixy2,
};
use core::mem::size_of_val;
use core::ptr::slice_from_raw_parts;
use embedded_hal::{delay::DelayNs, spi::SpiDevice};

/// Represents a region of colour tracked by PixyCam.
#[repr(C)]
pub struct Block {
    /// The signature of this block
    pub signature: u16,

    /// The x coordinate of this block on the camera.
    pub x: u16,
    /// The y coordinate of this block on the camera.
    pub y: u16,

    /// The width of this block (in pixels).
    pub width: u16,
    /// The height of this block (in pixels).
    pub height: u16,

    /// The angle of this block relative to the camera.
    pub angle: i16,
    /// The tracking index of this block.
    pub index: u8,
    /// How long has this been tracked for, in frames.
    /// Stops incrementing at 255.
    pub age: u8,
}

impl<Link: SpiDevice, W: DelayNs> Pixy2<Link, W> {
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
            self.send_packet(PacketType::RequestBlocks, &[signature_bitmap, max_blocks])
                .map_err(|e| OperationError::SendError(e))?;

            let (resp_type, resp_payload) = self
                .recv_packet()
                .map_err(|e| OperationError::RecvError(e))?;

            match resp_type {
                PacketType::ResponseBlocks => {
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
                PacketType::ResponseError => {
                    let result: PixyResultType = unsafe { *(resp_payload[0] as *const i8) }.into();

                    if let PixyResultType::Busy = result {
                        if wait {
                            continue;
                        } else {
                            return Err(OperationError::Busy);
                        }
                    } else if !matches!(result, PixyResultType::ProgramChanging)
                        && !matches!(result, PixyResultType::Ok)
                    {
                        return Err(OperationError::PixyError(result));
                    }
                }
                _ => {
                    return Err(OperationError::UnexpectedPacket {
                        got: resp_type,
                        expected: PacketType::ResponseBlocks,
                    });
                }
            }

            waiter.delay_us(500);
        }
    }
}
