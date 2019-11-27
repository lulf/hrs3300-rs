extern crate embedded_hal_mock as hal;
extern crate hrs3300;
use hal::i2c::Transaction as I2cTrans;
use hrs3300::{ConversionDelay, Gain};

mod common;
use common::{destroy, new, BitFlags as BF, Register as Reg, DEV_ADDR};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

macro_rules! set_test {
    ($name:ident, $method:ident, $register:ident, $value:expr $(, $arg:expr)*) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write(
                DEV_ADDR,
                vec![Reg::$register, $value],
            )];
            let mut sensor = new(&transactions);
            sensor.$method($($arg),*).unwrap();
            destroy(sensor);
        }
    };
}

set_test!(enable_hrs, enable_hrs, ENABLE, BF::HEN);
set_test!(disable_hrs, disable_hrs, ENABLE, 0);
macro_rules! set_conv_delay_test {
    ($name:ident, $arg:ident, $expected:expr) => {
        set_test!(
            $name,
            set_conversion_delay,
            ENABLE,
            $expected,
            ConversionDelay::$arg
        );
    };
}
set_conv_delay_test!(set_conv_delay_800, Ms800, 0);
set_conv_delay_test!(set_conv_delay_400, Ms400, 1 << 4);
set_conv_delay_test!(set_conv_delay_200, Ms200, 2 << 4);
set_conv_delay_test!(set_conv_delay_100, Ms100, 3 << 4);
set_conv_delay_test!(set_conv_delay_75, Ms75, 4 << 4);
set_conv_delay_test!(set_conv_delay_50, Ms50, 5 << 4);
set_conv_delay_test!(set_conv_delay_12_5, Ms12_5, 6 << 4);
set_conv_delay_test!(set_conv_delay_0, Ms0, 7 << 4);

set_test!(set_gain_1, set_gain, HGAIN, 0, Gain::One);
set_test!(set_gain_2, set_gain, HGAIN, 1 << 2, Gain::Two);
set_test!(set_gain_4, set_gain, HGAIN, 2 << 2, Gain::Four);
set_test!(set_gain_8, set_gain, HGAIN, 3 << 2, Gain::Eight);
set_test!(set_gain_64, set_gain, HGAIN, 4 << 2, Gain::SixtyFour);

macro_rules! get_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write_read(
                DEV_ADDR,
                vec![Reg::$register],
                vec![$value],
            )];
            let mut sensor = new(&transactions);
            let result = sensor.$method().unwrap();
            assert_eq!($expected, result);
            destroy(sensor);
        }
    };
}

get_test!(can_get_dev_id, device_id, ID, 0x21, 0x21);
