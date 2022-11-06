use std::result::Result;

use super::StringEncoding;
use crate::error::{PicoriError};

pub trait AsciiEncodingTrait {
    fn from_ascii(input: &[u8]) -> Result<String, PicoriError>;
}

impl AsciiEncodingTrait for String {
    fn from_ascii(input: &[u8]) -> Result<String, PicoriError> {
        let index = input.iter().position(|&x| x == 0).unwrap_or(input.len());
        let str = std::str::from_utf8(&input[..index])?;
        Ok(str.to_string())
    }
}

pub struct AsciiEncoding {}

impl StringEncoding for AsciiEncoding {
    fn decode_bytes(input: &[u8]) -> Result<String, PicoriError> { String::from_ascii(input) }
}
