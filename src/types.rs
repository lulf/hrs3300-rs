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
}
