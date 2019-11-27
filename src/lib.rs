//! This is a platform agnostic Rust driver for the HRS3300 heart rate sensor
//! using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!--
//! This driver allows you to:
//! -->
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The device
//!
//! HRSS3300 is an optical digital heart rate sensor/monitor featuring a 525nm
//! green LED and a reflection light detector for the PPG signal from the human
//! body.
//! The typical heart rate measurement samples the reflected PPG signal at
//! 25Hz then the results can be read via the I2C bus.
//!
//! Datasheet:
//! - [HRS3300](http://files.pine64.org/doc/datasheet/pinetime/HRS3300%20Heart%20Rate%20Sensor.pdf)
//!
//! <!--
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//! -->
#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// HRS3300 device driver
#[derive(Debug)]
pub struct Hrs3300<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
}

mod device_impl;
