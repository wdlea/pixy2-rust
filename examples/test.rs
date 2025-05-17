#![no_std]
#![no_main]

use arduino_hal::{delay_ms, spi::Settings, Delay, Spi};
use embedded_hal::spi::MODE_3;
use panic_halt as _;
use pixy2_rust::pixy::{OperationError, Pixy2};
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 115_200);

    let (spi, cs) = Spi::new(
        dp.SPI,
        pins.d52.into_output(),
        pins.d51.into_output(),
        pins.d50.into_pull_up_input(),
        pins.d53.into_output(),
        Settings {
            mode: MODE_3,
            ..Default::default()
        },
    );

    uwriteln!(serial, "Started up SPI").unwrap();

    let dev = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    uwriteln!(serial, "Got SPI device.").unwrap();

    let mut pixy = Pixy2::new(dev, arduino_hal::Delay::new(), &mut serial).unwrap_or_else(|e| {
        uwriteln!(serial, "Error connecting to pixy: {:?}", e);
        panic!("")
    });

    uwriteln!(serial, "Connected to PixyCam.").unwrap();

    loop {
        uwriteln!(serial, "Getting blocks:").unwrap();

        match pixy.get_blocks(true, u8::MAX, u8::MAX, &mut Delay::new()) {
            Ok(blocks) => {
                for block in blocks {
                    uwriteln!(
                        serial,
                        "Block: at ({}, {})(angle of {}), {} wide, {} tall with signature {} and age {}",
                        block.x,
                        block.y,
                        block.angle,
                        block.width,
                        block.height,
                        block.signature,
                        block.age
                    )
                    .unwrap();
                }
            }
            Err(OperationError::Busy) => {
                delay_ms(200); // just try again after a delay
            }
            Err(_) => panic!("Error getting blocks"),
        }

        delay_ms(10); // small delay to give PixyCam time to do other things
    }
}
