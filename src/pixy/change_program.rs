use std::io::Write;

use embedded_hal::delay::DelayNs;

use crate::link_type::LinkType;

use super::{Pixy2, operation_error::OperationError};

const REQUEST_CHANGE_PROGRAM: u8 = 0x02;
const MAX_PROGRAM_NAME_LENGTH: u8 = 33;

impl<Link: LinkType> Pixy2<Link> {
    /// Changes the program currently running on pixycam.
    /// I ported this not knowing what it does, there are
    /// no docs explaining what strings you should feed
    /// into `prog`.
    pub fn change_program(
        &mut self,
        prog: &str,
        waiter: &mut impl DelayNs,
    ) -> Result<(), OperationError<Link>> {
        loop {
            let mut buf = &mut self.buf[0..(MAX_PROGRAM_NAME_LENGTH as usize)];

            // Zero the buffer that will get sent
            buf.iter_mut().for_each(|i| *i = 0);

            buf.write(prog.as_bytes())
                .map_err(|e| OperationError::IOError(e))?;

            self.send_packet(REQUEST_CHANGE_PROGRAM, prog.as_bytes())
                .map_err(|e| OperationError::SendError(e))?;

            if let Ok((_resp_type, resp_payload)) = self.recv_packet() {
                let resp: u32 = unsafe { *resp_payload.as_ptr().cast() };
                if resp > 0 {
                    self.get_resolution()?;
                    return Ok(());
                }
            }

            waiter.delay_us(1_000);
        }
    }
}
