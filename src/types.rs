/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Wait time between each HRS conversion cycle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConversionDelay {
    /// 0 ms
    Ms0,
    /// 12.5 ms
    Ms12_5,
    /// 50 ms
    Ms50,
    /// 75 ms
    Ms75,
    /// 100 ms
    Ms100,
    /// 200 ms
    Ms200,
    /// 400 ms
    Ms400,
    /// 800 ms (default)
    Ms800,
}

impl Default for ConversionDelay {
    fn default() -> Self {
        ConversionDelay::Ms800
    }
}

/// HRS Gain
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gain {
    /// 1x (default)
    One,
    /// 2x
    Two,
    /// 4x
    Four,
    /// 8x
    Eight,
    /// 64x
    SixtyFour,
}

impl Default for Gain {
    fn default() -> Self {
        Gain::One
    }
}

/// Ambient light sensor resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlsResolution {
    /// 8 bits
    Bit8,
    /// 9 bits
    Bit9,
    /// 10 bits
    Bit10,
    /// 11 bits
    Bit11,
    /// 12 bits
    Bit12,
    /// 13 bits
    Bit13,
    /// 14 bits
    Bit14,
    /// 15 bits
    Bit15,
    /// 16 bits
    Bit16,
    /// 17 bits
    Bit17,
    /// 18 bits
    Bit18,
}

impl Default for AlsResolution {
    fn default() -> Self {
        AlsResolution::Bit8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! default_test {
        ($name:ident, $type:ident, $default:ident) => {
            #[test]
            fn $name() {
                assert_eq!($type::$default, $type::default());
            }
        };
    }

    default_test!(default_conv_delay, ConversionDelay, Ms800);
    default_test!(default_gain, Gain, One);
    default_test!(default_als_res, AlsResolution, Bit8);
}
