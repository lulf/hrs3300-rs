use {hal, Config, Error, Hrs3300};

const DEV_ADDR: u8 = 0x44;

struct Register;
impl Register {
    const ID: u8 = 0x00;
    const ENABLE: u8 = 0x01;
}

struct BitFlags;
impl BitFlags {
    const HEN: u8 = 1 << 7;
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl<I2C> Hrs3300<I2C> {
    /// Create new instance of the HRS3300 device.
    pub fn new(i2c: I2C) -> Self {
        Hrs3300 {
            i2c,
            enable: Config { bits: 0 },
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E> Hrs3300<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = E>,
{
    /// Enable the heart-rate sensor (HRS).
    pub fn enable_hrs(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable.with_high(BitFlags::HEN);
        self.set_enable(enable)
    }

    /// Disable the heart-rate sensor (HRS).
    pub fn disable_hrs(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable.with_low(BitFlags::HEN);
        self.set_enable(enable)
    }

    fn set_enable(&mut self, enable: Config) -> Result<(), Error<E>> {
        self.write_register(Register::ENABLE, enable.bits)?;
        self.enable = enable;
        Ok(())
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEV_ADDR, &[register, value])
            .map_err(Error::I2C)
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
