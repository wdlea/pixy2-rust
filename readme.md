# Pixy2 Rust
*An unofficial port of TPixy2.h and Pixy2CCC.h to Rust*

This library is a port of the [Pixy2 Library](https://github.com/charmedlabs/pixy2/)(Originally C++) to Rust, which was originally licensed under the [GNU General Public License v2](https://www.gnu.org/licenses/gpl-2.0.html).

## Functionality
This library implements **only CCC functionality**(`get_blocks` in original code). It also implements functions relating to the serial protocol, so it should be easy to implement the other *modes* as needed. 

## Compatibility
This library is only for use on **Little Endian** systems, I have tested it on an Arduino MEGA and it appears to work well. 

This library is only for Pixy2 cameras. It also only supports the SPI interface. 

> If using a device without a Chip Select(CS) pin (such as using the group of 6 pins with the adapter cable on some Arduino boards), make sure to configure PixyCam to use **Arduino ISCP SPI**. 

## Contributions 
If you would like to add some missing functionality, feel free to submit a PR. 

## Example

This example is for devices supported by `arduino_hal`. You can run it from the root of this library via `cargo r --example test -- -P [your_port]`, you may need to install some tools first.
```rust
#![no_std]
#![no_main]

use arduino_hal::{Delay, Spi, delay_ms, spi::Settings};
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

    let mut pixy = Pixy2::new(dev, arduino_hal::Delay::new()).unwrap_or_else(|e| {
        uwriteln!(serial, "Error connecting to pixy: {:?}", e).unwrap();
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
            Err(e) => {
                uwriteln!(serial, "Oopsie: {:?}", e).unwrap();
                panic!("Error getting blocks");
            }
        }

        delay_ms(10); // small delay to give PixyCam time to do other things
    }
}

```