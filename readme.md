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