use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyDataError,
    OverflowError,
    IncorrectLenError,
    IOError,
    TypeError,
    UnsignableMessage,
}

impl Error {
    pub fn to_jsval(&self) -> JsValue {
        use Error::*;

        match self {
            EmptyDataError | OverflowError | IncorrectLenError | IOError | TypeError
            | UnsignableMessage => JsValue::from(self.to_string()),
        }
    }
}

impl From<JsValue> for Error {
    fn from(_: JsValue) -> Self {
        Self::TypeError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDataError => {
                write!(f, "Empty array cannot be hashed")
            }
            Self::OverflowError => {
                write!(f, "Field overflow")
            }
            Self::IOError => {
                write!(f, "Bytes reading failed")
            }
            Self::IncorrectLenError => {
                write!(f, "Incorrect array length")
            }
            Self::TypeError => {
                write!(f, "Incorrect type")
            }
            Self::UnsignableMessage => {
                write!(f, "Msg is not signable")
            }
        }
    }
}
