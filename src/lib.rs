pub trait CRC<N>: Default
where
    N: std::ops::Shl + std::ops::ShlAssign + std::ops::BitXor + std::ops::BitXorAssign,
{
    fn create(polynomial: N) -> Self;
    fn update(&mut self, data: &[u8]);
    fn finalize(&self) -> N;
}

pub struct CRC8 {
    crc: u8,
    polynomial: u8,
}

impl CRC<u8> for CRC8 {
    fn update(&mut self, data: &[u8]) {
        for ibyte in data {
            self.crc ^= *ibyte;
            for _bit in 0..8u8 {
                if self.crc & 0x80 != 0 {
                    // MSB is set so shift + XOR polynomial
                    self.crc = (self.crc << 1) ^ self.polynomial;
                } else {
                    // MSB is not set so just shift
                    self.crc <<= 1;
                }
            }
        }
    }

    fn finalize(&self) -> u8 {
        self.crc ^ u8::MAX
    }

    fn create(polynomial: u8) -> Self {
        Self {
            crc: u8::MAX,
            polynomial,
        }
    }
}

impl std::default::Default for CRC8 {
    fn default() -> Self {
        // TODO: check default polynomial
        Self::create(0b00000111)
    }
}

pub struct CRC16 {
    crc: u16,
    polynomial: u16,
}

impl CRC<u16> for CRC16 {
    fn update(&mut self, data: &[u8]) {
        for ibyte in data {
            self.crc ^= (*ibyte as u16) << 8;
            for _bit in 0..8u8 {
                if self.crc & 0x8000 != 0 {
                    // MSB is set so shift + XOR polynomial
                    self.crc = (self.crc << 1) ^ self.polynomial;
                } else {
                    // MSB is not set so just shift
                    self.crc <<= 1;
                }
            }
        }
    }

    fn finalize(&self) -> u16 {
        self.crc ^ u16::MAX
    }

    fn create(polynomial: u16) -> Self {
        Self {
            crc: u16::MAX,
            polynomial,
        }
    }
}

impl std::default::Default for CRC16 {
    fn default() -> Self {
        // TODO: check default polynomial
        Self::create(0b1000_0000_0000_0101)
    }
}

struct CRC32 {
    crc: u32,
    polynomial: u32,
}

impl CRC<u32> for CRC32 {
    fn update(&mut self, data: &[u8]) {
        for ibyte in data {
            self.crc ^= (*ibyte as u32) << 24;
            for _bit in 0..8u8 {
                if self.crc & 0x8000_0000 != 0 {
                    // MSB is set so shift + XOR polynomial
                    self.crc = (self.crc << 1) ^ self.polynomial;
                } else {
                    // MSB is not set so just shift
                    self.crc <<= 1;
                }
            }
        }
    }

    fn finalize(&self) -> u32 {
        self.crc ^ u32::MAX
    }

    fn create(polynomial: u32) -> Self {
        Self {
            crc: u32::MAX,
            polynomial,
        }
    }
}

impl std::default::Default for CRC32 {
    fn default() -> Self {
        // TODO: check default polynomial
        Self::create(0b0000_0100_1100_0001_0001_1101_1011_0111)
    }
}

struct CRC64 {
    crc: u64,
    polynomial: u64,
}

impl CRC<u64> for CRC64 {
    fn update(&mut self, data: &[u8]) {
        for ibyte in data {
            self.crc ^= (*ibyte as u64) << 56;
            for _bit in 0..8u8 {
                if self.crc & 0x8000_0000_0000_0000 != 0 {
                    // MSB is set so shift + XOR polynomial
                    self.crc = (self.crc << 1) ^ self.polynomial;
                } else {
                    // MSB is not set so just shift
                    self.crc <<= 1;
                }
            }
        }
    }

    fn finalize(&self) -> u64 {
        self.crc ^ u64::MAX
    }

    fn create(polynomial: u64) -> Self {
        Self {
            crc: u64::MAX,
            polynomial,
        }
    }
}

impl std::default::Default for CRC64 {
    fn default() -> Self {
        // TODO: check default polynomial
        Self::create(
            0b0100_0010_1111_0000_1110_0001_1110_1011_1010_1001_1110_1010_0011_0110_1001_0011,
        )
    }
}

struct CRC128 {
    crc: u128,
    polynomial: u128,
}

impl CRC<u128> for CRC128 {
    fn update(&mut self, data: &[u8]) {
        for ibyte in data {
            self.crc ^= (*ibyte as u128) << 120;
            for _bit in 0..8u8 {
                if self.crc & 0x8000_0000_0000_0000_0000_0000_0000_0000 != 0 {
                    // MSB is set so shift + XOR polynomial
                    self.crc = (self.crc << 1) ^ self.polynomial;
                } else {
                    // MSB is not set so just shift
                    self.crc <<= 1;
                }
            }
        }
    }

    fn finalize(&self) -> u128 {
        self.crc ^ u128::MAX
    }

    fn create(polynomial: u128) -> Self {
        Self {
            crc: u128::MAX,
            polynomial,
        }
    }
}

impl std::default::Default for CRC128 {
    fn default() -> Self {
        // TODO: check default polynomial
        Self::create(0b1110_0011_1100_0011_1101_0101_1010_0111_1110_1001_1111_0111_1101_0100_1110_0001_1111_0011_1111_0000_1111_1011_1010_1011_0110_0101_1100_0111_1000_1001_0001)
    }
}

#[cfg(test)]
mod tests {
    use super::{CRC, CRC128, CRC16, CRC32, CRC64, CRC8};

    const TEST_DATA: &[u8] = b"hello world";

    #[test]
    fn crc8_test() {
        let mut crc = CRC8::default();
        crc.update(TEST_DATA);
        let crc = crc.finalize();
        assert!(crc == 0x94, "{:#X}", crc);
    }

    #[test]
    fn crc16_test() {
        let mut crc = CRC16::default();
        crc.update(TEST_DATA);
        let crc = crc.finalize();
        assert!(crc == 0xC814, "{:#X}", crc);
    }

    #[test]
    fn crc32_test() {
        let mut crc = CRC32::default();
        crc.update(TEST_DATA);
        let crc = crc.finalize();
        assert!(crc == 0x44F71378, "{:#X}", crc);
    }

    #[test]
    fn crc64_test() {
        let mut crc = CRC64::default();
        crc.update(TEST_DATA);
        let crc = crc.finalize();
        assert!(crc == 0xC287020321943B9D, "{:#X}", crc);
    }

    #[test]
    fn crc128_test() {
        let mut crc = CRC128::default();
        crc.update(TEST_DATA);
        let crc = crc.finalize();
        assert!(crc == 0x1B004A91C7EF19134E779C0AC320AD8C, "{:#X}", crc);
    }
}
