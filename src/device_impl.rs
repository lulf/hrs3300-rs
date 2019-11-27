use {hal, Error, Hrs3300};

const DEV_ADDR: u8 = 0x44;

struct Register;
impl Register {
    const ID: u8 = 0x00;
}

impl<I2C> Hrs3300<I2C> {
    /// Create new instance of the HRS3300 device.
    pub fn new(i2c: I2C) -> Self {
        Hrs3300 { i2c }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E> Hrs3300<I2C>
where
    I2C: hal::blocking::i2c::WriteRead<Error = E>,
{
    /// Read the device ID (0x21).
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::ID)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEV_ADDR, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }
}
