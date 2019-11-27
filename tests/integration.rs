extern crate embedded_hal_mock as hal;
extern crate hrs3300;
use hal::i2c::Transaction as I2cTrans;

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

set_test!(can_enable_hrs, enable_hrs, ENABLE, BF::HEN);
set_test!(can_disable_hrs, disable_hrs, ENABLE, 0);

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
