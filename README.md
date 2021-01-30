# Rust HRS3300 Heart Rate Sensor Driver

[![crates.io](https://img.shields.io/crates/v/hrs3300.svg)](https://crates.io/crates/hrs3300)
[![Docs](https://docs.rs/hrs3300/badge.svg)](https://docs.rs/hrs3300)
[![Build Status](https://github.com/eldruin/hrs3300-rs/workflows/Build/badge.svg)](https://github.com/eldruin/hrs3300-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/hrs3300-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/hrs3300-rs?branch=master)

This is a platform agnostic Rust driver for the HRS3300 optical heart rate
sensor using the [`embedded-hal`] traits.

This driver allows you to:
- Enable/disable heart rate sensor. See: `enable_hrs()`.
- Enable/disable oscillator. See: `enable_oscillator()`.
- Initialize the device. See: `init()`.
- Set the conversion delay. See: `set_conversion_delay()`.
- Set the gain. See: `set_gain()`.
- Set the ambient light sensor resolution. See: `set_als_resolution()`.
- Set the LED current. See: `set_led_current()`.
- Read the device id. See: `device_id()`.
- Read the last heart rate sensor measurement. See: `read_hrs()`.
- Read the last ambient light sensor measurement. See: `read_als()`.
- Write/Read a register with a custom value. See: `write_register()`.

<!-- TODO
[Introductory blog post]()
-->

## The device

HRSS3300 is an optical digital heart rate sensor/monitor featuring a 525nm
green LED and a reflection light detector for the PPG signal from the human
body.
The typical heart rate measurement samples the reflected PPG signal at
25Hz then the results can be read via the I2C bus.

Datasheet:
- [HRS3300](http://files.pine64.org/doc/datasheet/pinetime/HRS3300%20Heart%20Rate%20Sensor.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

```rust
extern crate hrs3300;
extern crate linux_embedded_hal as hal;
use hrs3300::Hrs3300;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Hrs3300::new(dev);
    sensor.init().unwrap();
    sensor.enable_hrs().unwrap();
    sensor.enable_oscillator().unwrap();
    loop {
        let hrs = sensor.read_hrs().unwrap();
        let als = sensor.read_als().unwrap();
        println!("HRS: {}, ALS: {}", hrs, als);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/hrs3300-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
