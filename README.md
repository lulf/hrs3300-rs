# Rust HRS3300 Heart Rate Sensor Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/hrs3300.svg)](https://crates.io/crates/hrs3300)
[![Docs](https://docs.rs/hrs3300/badge.svg)](https://docs.rs/hrs3300)
-->
[![Build Status](https://travis-ci.org/eldruin/hrs3300-rs.svg?branch=master)](https://travis-ci.org/eldruin/hrs3300-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/hrs3300-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/hrs3300-rs?branch=master)

This is a platform agnostic Rust driver for the HRS3300 optical heart rate
sensor using the [`embedded-hal`] traits.

<!-- TODO
This driver allows you to:
-->
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

<!-- TODO
## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
...
```
-->

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
