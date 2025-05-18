#![no_std]
#![no_main]

use arduino_hal::{spi::{DataOrder, Settings}, Delay, Spi};
use embedded_hal::{
    digital::OutputPin,
    spi::{Mode, Phase, Polarity, SpiDevice, MODE_1, MODE_3},
};
use panic_halt as _;
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
            mode: Mode{
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnSecondTransition,
            },
            data_order: DataOrder::MostSignificantFirst,
            ..Default::default()
        },
    );

    uwriteln!(serial, "Started up SPI").unwrap();

    let mut dev = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    uwriteln!(serial, "Got SPI device.").unwrap();

    loop {
        let version_request = [0xae, 0xc1, 0x0e, 0x00];
        let mut buf = [0u8; 6 + 16];

        dev.write(&version_request)
            .or_else(|_| uwriteln!(serial, "Couldn't write"));

        arduino_hal::delay_ms(1);

        dev.read(&mut buf)
            .or_else(|_| uwriteln!(serial, "Couldn't read"));

        uwriteln!(serial, "Received the following bytes: {:?}", buf);

        arduino_hal::delay_ms(100);
    }
}
