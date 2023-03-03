use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::java::FieldType;
use crate::java::FieldType::{Array, Byte, Char, Double, Float, Int, Long, Reference, Short};

pub struct TypeParser<'a> {
    descriptor: &'a str,
    position: usize,
}

impl<'a> TypeParser<'a> {
    pub fn new(descriptor: &'a str) -> Self {
        TypeParser { descriptor, position: 0 }
    }

    pub fn next(&mut self) -> Result<FieldType, ParseError> {
        if self.position >= self.descriptor.len() {
            return Err(ParseError(self.descriptor.to_string()));
        }
        let first: &str = self.descriptor.get(self.position..self.position + 1).unwrap();
        match first {
            "Z" => {
                self.position += 1;
                return Ok(FieldType::Boolean);
            }
            "B" => {
                self.position += 1;
                Ok(Byte)
            }
            "C" => {
                self.position += 1;
                Ok(Char)
            }
            "S" => {
                self.position += 1;
                Ok(Short)
            }
            "I" => {
                self.position += 1;
                Ok(Int)
            }
            "J" => {
                self.position += 1;
                Ok(Long)
            }
            "F" => {
                self.position += 1;
                Ok(Float)
            }
            "D" => {
                self.position += 1;
                Ok(Double)
            }
            "L" => {
                self.position += 1;
                let name_and_rest = &self.descriptor[self.position..];
                let semicolon_pos = name_and_rest.find(';').ok_or(ParseError(self.descriptor.to_string()))?;
                let name = name_and_rest[..semicolon_pos].replace('/', ".");
                if name.is_empty() {
                    return Err(ParseError(self.descriptor.to_string()));
                }
                self.position += semicolon_pos + 1;
                Ok(Reference(name))
            }
            "[" => {
                self.position += 1;
                let part = self.next()?;
                Ok(Array(Box::new(part)))
            }
            _ => {
                Err(ParseError(self.descriptor.to_string()))
            }
        }
    }

    pub fn expect_char(&mut self, char: char) -> Result<(), ParseError> {
        if self.position >= self.descriptor.len() {
            return Err(ParseError(self.descriptor.to_string()));
        }
        if self.descriptor[self.position..].starts_with(char) {
            self.position += 1;
            Ok(())
        } else {
            Err(ParseError(self.descriptor.to_string()))
        }
    }

    pub fn expect_end(&self) -> Result<(), ParseError> {
        if self.position != self.descriptor.len() {
            return Err(ParseError(self.descriptor.to_string()))
        }
        return Ok(())
    }

    pub fn peek(&self) -> Result<char, ParseError> {
        if self.position >= self.descriptor.len() {
            return Err(ParseError(self.descriptor.to_string()));
        }
        let current = &self.descriptor[self.position..];
        Ok(current.chars().next().unwrap())
    }
}

/// An error when parsing a field or method type.
#[derive(Debug)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse descriptor '{}'", &self.0)
    }
}

impl Error for ParseError {}