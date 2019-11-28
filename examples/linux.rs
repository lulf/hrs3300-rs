extern crate hrs3300;
extern crate linux_embedded_hal as hal;
use hrs3300::Hrs3300;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Hrs3300::new(dev);
    sensor.enable_hrs().unwrap();
    loop {
        let hrs = sensor.read_hrs().unwrap();
        let als = sensor.read_als().unwrap();
        println!("HRS: {}, ALS: {}", hrs, als);
    }
}
