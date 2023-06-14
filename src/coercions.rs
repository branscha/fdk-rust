use crate::FunctionError;
use serde::{Deserialize, Serialize};

/// ContentType represents the supported content types in the FDK.
#[derive(Clone, Debug)]
pub enum ContentType {
    JSON,
    YAML,
    XML,
    Plain,
    URLEncoded,
}

const JSON_MIME : &str = "application/json";
const TEXT_MIME : &str = "text/plain";
const FORM_MIME : &str = "application/x-www-form-urlencoded";
const XML_TEXT_MIME : &str = "text/xml";
const XML_APP_MIME : &str = "application/xml";
const YAML_TEXT_MIME : &str = "text/yaml";
const YAML_APP_MIME : &str = "application/yaml";

impl ContentType {
    pub fn from_str(s: &str) -> Self {
        match s {
            JSON_MIME => ContentType::JSON,
            YAML_TEXT_MIME | YAML_APP_MIME => ContentType::YAML,
            XML_TEXT_MIME | XML_APP_MIME => ContentType::XML,
            TEXT_MIME => ContentType::Plain,
            FORM_MIME => ContentType::URLEncoded,
            _ => ContentType::JSON,
        }
    }

    pub fn as_header_value(&self) -> String {
        match self {
            Self::JSON => String::from(JSON_MIME),
            Self::YAML => String::from(YAML_TEXT_MIME),
            Self::XML => String::from(XML_APP_MIME),
            Self::Plain => String::from(TEXT_MIME),
            Self::URLEncoded => String::from(FORM_MIME),
        }
    }
}

/// An `InputCoercible` type can be generated from a `Vec<u8>`.
pub trait InputCoercible: Sized {
    fn try_decode_plain(input: Vec<u8>) -> Result<Self, FunctionError>;
    fn try_decode_json(input: Vec<u8>) -> Result<Self, FunctionError>;
    fn try_decode_xml(input: Vec<u8>) -> Result<Self, FunctionError>;
    fn try_decode_yaml(input: Vec<u8>) -> Result<Self, FunctionError>;
    fn try_decode_urlencoded(input: Vec<u8>) -> Result<Self, FunctionError>;
}

/// An `OutputCoercible` type can be converted to a `Vec<u8>`.
pub trait OutputCoercible: Sized {
    fn try_encode_json(self) -> Result<Vec<u8>, FunctionError>;
    fn try_encode_xml(self) -> Result<Vec<u8>, FunctionError>;
    fn try_encode_yaml(self) -> Result<Vec<u8>, FunctionError>;
    fn try_encode_plain(self) -> Result<Vec<u8>, FunctionError>;
    fn try_encode_urlencoded(self) -> Result<Vec<u8>, FunctionError>;
}

impl<T: for<'de> Deserialize<'de>> InputCoercible for T {
    fn try_decode_plain(input: Vec<u8>) -> Result<Self, FunctionError> {
        match serde_plain::from_str(&input.iter().map(|&v| v as char).collect::<String>()) {
            Ok(t) => Ok(t),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }

    fn try_decode_json(input: Vec<u8>) -> Result<Self, FunctionError> {
        match serde_json::from_slice(input.as_slice()) {
            Ok(t) => Ok(t),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }

    fn try_decode_xml(input: Vec<u8>) -> Result<Self, FunctionError> {
        match serde_xml_rs::from_str(&input.iter().map(|&v| v as char).collect::<String>()) {
            Ok(t) => Ok(t),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }

    fn try_decode_yaml(input: Vec<u8>) -> Result<Self, FunctionError> {
        match serde_yaml::from_slice(input.as_slice()) {
            Ok(t) => Ok(t),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }

    fn try_decode_urlencoded(input: Vec<u8>) -> Result<Self, FunctionError> {
        match serde_urlencoded::from_str(&input.iter().map(|&v| v as char).collect::<String>()) {
            Ok(t) => Ok(t),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }
}

impl<T: Serialize> OutputCoercible for T {
    fn try_encode_json(self) -> Result<Vec<u8>, FunctionError> {
        match serde_json::to_vec(&self) {
            Ok(vector) => Ok(vector),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }
    fn try_encode_xml(self) -> Result<Vec<u8>, FunctionError> {
        match serde_xml_rs::to_string(&self) {
            Ok(vector) => Ok(vector.chars().map(|ch| ch as u8).collect()),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }
    fn try_encode_yaml(self) -> Result<Vec<u8>, FunctionError> {
        match serde_yaml::to_vec(&self) {
            Ok(vector) => Ok(vector),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }

    fn try_encode_plain(self) -> Result<Vec<u8>, FunctionError> {
        match serde_plain::to_string(&self) {
            Ok(vector) => Ok(vector.chars().map(|ch| ch as u8).collect()),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }

    fn try_encode_urlencoded(self) -> Result<Vec<u8>, FunctionError> {
        match serde_urlencoded::to_string(&self) {
            Ok(vector) => Ok(vector.chars().map(|ch| ch as u8).collect()),
            Err(e) => Err(FunctionError::Coercion {
                inner: e.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {

use super::*;

    // Parsing empty input should result in empty string.
    #[test]
    fn empty_text_plain() {
        let res = String::try_decode_plain(vec![]);
        println!("{:?}", res);
        match res {
            Ok(str) => assert_eq!("", str),
            Err(err) => panic!("{:?}", err)
        }
    }

    // #[test]
    // fn empty_text_json() {
    //     let res = Option::<String>::try_decode_json(vec![]);
    //     println!("{:?}", res);
    //     match res {
    //         Ok(option) => assert!(option.is_none()),
    //         Err(err) => panic!("{:?}", err)
    //     }
    // }

}
