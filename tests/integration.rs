extern crate embedded_hal_mock as hal;
extern crate hrs3300;
use hal::i2c::Transaction as I2cTrans;
use hrs3300::{AlsResolution as Res, ConversionDelay, Gain, LedCurrent};

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

set_test!(set_als_res8, set_als_resolution, RESOLUTION, 0, Res::Bit8);
set_test!(set_als_res9, set_als_resolution, RESOLUTION, 1, Res::Bit9);
set_test!(set_als_res10, set_als_resolution, RESOLUTION, 2, Res::Bit10);
set_test!(set_als_res11, set_als_resolution, RESOLUTION, 3, Res::Bit11);
set_test!(set_als_res12, set_als_resolution, RESOLUTION, 4, Res::Bit12);
set_test!(set_als_res13, set_als_resolution, RESOLUTION, 5, Res::Bit13);
set_test!(set_als_res14, set_als_resolution, RESOLUTION, 6, Res::Bit14);
set_test!(set_als_res15, set_als_resolution, RESOLUTION, 7, Res::Bit15);
set_test!(set_als_res16, set_als_resolution, RESOLUTION, 8, Res::Bit16);
set_test!(set_als_res17, set_als_resolution, RESOLUTION, 9, Res::Bit17);
set_test!(set_alsres18, set_als_resolution, RESOLUTION, 10, Res::Bit18);

macro_rules! set_led_current_test {
    ($name:ident, $led_current:ident, $enable:expr, $pdriver:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(DEV_ADDR, vec![Reg::ENABLE, $enable]),
                I2cTrans::write(DEV_ADDR, vec![Reg::PDRIVER, $pdriver]),
            ];
            let mut sensor = new(&transactions);
            sensor.set_led_current(LedCurrent::$led_current).unwrap();
            destroy(sensor);
        }
    };
}

set_led_current_test!(set_led_curr_12_5, Ma12_5, 0, 0);
set_led_current_test!(set_led_curr_20, Ma20, 0, BF::PDRIVE0);
set_led_current_test!(set_led_curr_30, Ma30, BF::PDRIVE1, 0);
set_led_current_test!(set_led_curr_40, Ma40, BF::PDRIVE1, BF::PDRIVE0);

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

#[test]
fn can_read_hrs() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![0x09], vec![0b1001_0110, 0b1111_1010]),
        I2cTrans::write_read(DEV_ADDR, vec![0x0F], vec![0b1110_0101]),
    ];
    let mut sensor = new(&transactions);
    let result = sensor.read_hrs().unwrap();
    assert_eq!(0b10_1001_0110_1010_0101, result);
    destroy(sensor);
}

#[test]
fn can_read_als() {
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![0x08], vec![0b1001_0110]),
        I2cTrans::write_read(DEV_ADDR, vec![0x0D], vec![0b1010_1010, 0b1111_1101]),
    ];
    let mut sensor = new(&transactions);
    let result = sensor.read_als().unwrap();
    assert_eq!(0b01_0101_0100_1011_0101, result);
    destroy(sensor);
}
