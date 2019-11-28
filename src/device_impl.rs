use {hal, AlsResolution, Config, ConversionDelay, Error, Gain, Hrs3300};

const DEV_ADDR: u8 = 0x44;

struct Register;
impl Register {
    const ID: u8 = 0x00;
    const ENABLE: u8 = 0x01;
    const RESOLUTION: u8 = 0x16;
    const HGAIN: u8 = 0x17;
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

    /// Set the HRS conversion delay (waiting time between conversion cycles)
    pub fn set_conversion_delay(&mut self, delay: ConversionDelay) -> Result<(), Error<E>> {
        let delay_bits = match delay {
            ConversionDelay::Ms800 => 0,
            ConversionDelay::Ms400 => 1,
            ConversionDelay::Ms200 => 2,
            ConversionDelay::Ms100 => 3,
            ConversionDelay::Ms75 => 4,
            ConversionDelay::Ms50 => 5,
            ConversionDelay::Ms12_5 => 6,
            ConversionDelay::Ms0 => 7,
        };
        let bits = (self.enable.bits & !(7 << 4)) | (delay_bits << 4);
        self.set_enable(Config { bits })
    }

    /// Set the HRS ADC gain
    pub fn set_gain(&mut self, gain: Gain) -> Result<(), Error<E>> {
        let bits = match gain {
            Gain::One => 0,
            Gain::Two => 1 << 2,
            Gain::Four => 2 << 2,
            Gain::Eight => 3 << 2,
            Gain::SixtyFour => 4 << 2,
        };
        self.write_register(Register::HGAIN, bits)
    }

    /// Set the ambient light sensor ADC resolution
    pub fn set_als_resolution(&mut self, resolution: AlsResolution) -> Result<(), Error<E>> {
        let bits = match resolution {
            AlsResolution::Bit8 => 0,
            AlsResolution::Bit9 => 1,
            AlsResolution::Bit10 => 2,
            AlsResolution::Bit11 => 3,
            AlsResolution::Bit12 => 4,
            AlsResolution::Bit13 => 5,
            AlsResolution::Bit14 => 6,
            AlsResolution::Bit15 => 7,
            AlsResolution::Bit16 => 8,
            AlsResolution::Bit17 => 9,
            AlsResolution::Bit18 => 10,
        };
        self.write_register(Register::RESOLUTION, bits)
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

    /// Read heart rate sensor measurement (CH0)
    pub fn read_hrs(&mut self) -> Result<u32, Error<E>> {
        let mut data_09_0a = [0, 0];
        self.i2c
            .write_read(DEV_ADDR, &[0x09], &mut data_09_0a)
            .map_err(Error::I2C)?;
        let data_0f = self.read_register(0x0F)?;

        Ok(u32::from(data_0f & 0xF)
            | u32::from((data_09_0a[1] & 0xF) << 4)
            | u32::from(data_09_0a[0]) << 8
            | u32::from(data_0f & 0x30) << 12)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEV_ADDR, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }
}
