use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::FieldType::{Array, Byte, Char, Double, Float, Int, Long, Reference, Short};

/// A representation of the type of a class, instance or local variable.
///
/// For more information, see
/// [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
#[derive(PartialEq, Debug)]
pub enum FieldType {
    /// The type `boolean`, true or false.
    Boolean,
    /// The type `byte`, a signed byte.
    Byte,
    /// The type `char`, a UTF-16 character.
    Char,
    /// The type `short`, a signed short.
    Short,
    /// The type `int`, an integer.
    Int,
    /// The type `long`, a 64 bit signed integer.
    Long,
    /// The type `float`, a single-precision floating-point value.
    Float,
    /// The type `double`, a double-precision floating-point value.
    Double,
    /// An instance of a class, named by `String`.
    Reference(String),
    /// A one dimensional array, of the given field type.
    Array(Box<FieldType>),
}

impl FieldType {
    pub fn from_descriptor(descriptor: &str) -> Result<Self, ParseError> {
        let mut parser = State { descriptor, position: 0 };
        let field_type = parser.next()?;
        if parser.position != descriptor.len() {
            return Err(ParseError(descriptor.to_string()));
        }
        Ok(field_type)
    }
}

/// A representation of the type of a method.
///
/// For more information, see
/// [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.3).
pub struct MethodType {
    parameters: Vec<FieldType>,
    returns: Option<FieldType>,
}

struct State<'a> {
    descriptor: &'a str,
    position: usize,
}

impl<'a> State<'a> {
    fn next(&mut self) -> Result<FieldType, ParseError> {
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
                let name = name_and_rest[..semicolon_pos].to_string();
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
}

#[derive(Debug)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse descriptor '{}'", &self.0)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use crate::FieldType::Boolean;

    use super::*;

    #[test]
    fn field_type_from_descriptor_primitive() {
        assert_eq!(FieldType::from_descriptor("Z").unwrap(), Boolean);
        assert_eq!(FieldType::from_descriptor("B").unwrap(), Byte);
        assert_eq!(FieldType::from_descriptor("C").unwrap(), Char);
        assert_eq!(FieldType::from_descriptor("S").unwrap(), Short);
        assert_eq!(FieldType::from_descriptor("I").unwrap(), Int);
        assert_eq!(FieldType::from_descriptor("J").unwrap(), Long);
        assert_eq!(FieldType::from_descriptor("F").unwrap(), Float);
        assert_eq!(FieldType::from_descriptor("D").unwrap(), Double);
    }

    #[test]
    fn field_type_from_descriptor_class() {
        assert_eq!(FieldType::from_descriptor("Labc;").unwrap(), Reference("abc".to_string()));
        assert_eq!(FieldType::from_descriptor("Ljava/thing;").unwrap(), Reference("java/thing".to_string()));
    }

    #[test]
    fn field_type_from_descriptor_array() {
        assert_eq!(FieldType::from_descriptor("[Z").unwrap(), Array(Box::new(Boolean)));
        assert_eq!(FieldType::from_descriptor("[Ljava;").unwrap(), Array(Box::new(Reference("java".to_string()))));
        assert_eq!(FieldType::from_descriptor("[[Z").unwrap(), Array(Box::new(Array(Box::new(Boolean)))));
    }

    #[test]
    fn field_type_from_descriptor_errors() {
        assert!(FieldType::from_descriptor("").is_err());
        assert!(FieldType::from_descriptor("A").is_err());
        assert!(FieldType::from_descriptor("L").is_err());
        assert!(FieldType::from_descriptor("Labc").is_err());
        assert!(FieldType::from_descriptor("L;").is_err());
        assert!(FieldType::from_descriptor("[").is_err());
        assert!(FieldType::from_descriptor("[L").is_err());
        assert!(FieldType::from_descriptor("[L;").is_err());
    }
}