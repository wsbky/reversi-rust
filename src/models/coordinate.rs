use std::error;
use std::fmt;

pub struct Coordinate {
    pub bit: u64,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Result<Coordinate> {
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            Ok(Coordinate {
                bit: 0x8000000000000000 >> (x + y * 8),
            })
        } else {
            Err(Error::OutOfRangeError(x, y))
        }
    }

    pub fn transfer(&self, k: u32) -> Self {
        match k {
            0 => Self {
                bit: (self.bit << 8) & 0xffffffffffffff00,
            },
            1 => Self {
                bit: (self.bit << 7) & 0x7f7f7f7f7f7f7f00,
            },
            2 => Self {
                bit: (self.bit >> 1) & 0x7f7f7f7f7f7f7f7f,
            },
            3 => Self {
                bit: (self.bit >> 9) & 0x007f7f7f7f7f7f7f,
            },
            4 => Self {
                bit: (self.bit >> 8) & 0x00ffffffffffffff,
            },
            5 => Self {
                bit: (self.bit >> 7) & 0x00fefefefefefefe,
            },
            6 => Self {
                bit: (self.bit << 1) & 0xfefefefefefefefe,
            },
            7 => Self {
                bit: (self.bit << 9) & 0xfefefefefefefe00,
            },
            _ => Self { bit: 0 },
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OutOfRangeError(i32, i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Error::*;
        match self {
            OutOfRangeError(x, y) => write!(
                f,
                "OutOfRangeError: coordinate (x: {}, y: {}) is out of the board.",
                x, y
            ),
        }
    }
}

impl error::Error for Error {}
