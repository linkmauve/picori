use std::result::Result;

use super::StringDecoder;
use crate::error::PicoriError;
use crate::error::StringEncodingError::*;

pub struct AsciiDecoder {}

impl AsciiDecoder {
    pub fn decode_byte(byte: u8) -> Option<char> {
        match byte {
            // ASCII character
            0x00..=0x7f => Some(byte as char),
            // Invalid
            _ => None,
        }
    }
}

impl StringDecoder for AsciiDecoder {
    fn decode_iterator<T>(input: T) -> Result<String, PicoriError>
    where
        T: Iterator<Item = u8>,
    {
        let mut output = String::new();
        let mut iter = input.peekable();

        while let Some(byte) = iter.next() {
            match Self::decode_byte(byte) {
                Some(c) => output.push(c),
                None => return Err(InvalidByte(byte).into()),
            }
        }

        Ok(output)
    }

    fn decode_until_zero_iterator<T>(input: T) -> Result<String, PicoriError>
    where
        T: Iterator<Item = u8>,
    {
        Self::decode_iterator(input.take_while(|b| *b != 0))
    }
}
