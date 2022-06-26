use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    HashError,
    EmptyDataError,
    OverflowError,
    IncorrectLenError,
    IOError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HashError => {
                write!(f, "hash error ocurred")
            }
            Self::EmptyDataError => {
                write!(f, "emtpy data error")
            }
            Self::OverflowError => {
                write!(f, "field overflow error")
            }
            Self::IOError => {
                write!(f, "bytes read error")
            }
            Self::IncorrectLenError => {
                write!(f, "incorrect array len error")
            }
        }
    }
}
