use std::fmt;
use wasm_bindgen::prelude::*;
// use js_sys::JsValue;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub enum Error {
    HashError,
    EmptyDataError,
    OverflowError,
    IncorrectLenError,
    IOError,
}

impl Error {
    pub fn to_jsval(&self) -> JsValue {
        use Error::*;

        match self {
            HashError | EmptyDataError | OverflowError | IncorrectLenError | IOError => {
                JsValue::from(self.to_string())
            }
        }
    }
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
