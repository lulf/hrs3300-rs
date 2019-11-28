use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use hrs3300::Hrs3300;

pub const DEV_ADDR: u8 = 0x44;

pub struct Register;
impl Register {
    pub const ID: u8 = 0x00;
    pub const ENABLE: u8 = 0x01;
    pub const RESOLUTION: u8 = 0x16;
    pub const HGAIN: u8 = 0x17;
}

pub struct BitFlags;
impl BitFlags {
    pub const HEN: u8 = 1 << 7;
}

pub fn new(transactions: &[I2cTrans]) -> Hrs3300<I2cMock> {
    Hrs3300::new(I2cMock::new(transactions))
}

pub fn destroy(sensor: Hrs3300<I2cMock>) {
    sensor.destroy().done();
}
