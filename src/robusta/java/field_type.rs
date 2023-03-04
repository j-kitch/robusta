use std::fmt::Debug;

use crate::java::{Double, Float, Int, Long, ParseError, Reference, Value};
use crate::java::type_parser::TypeParser;

/// A representation of the type of a class, instance or local variable.
///
/// For more information, see
/// [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
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
    /// Parse the `FieldType` instance from a field descriptor.
    ///
    /// For further reference, see [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
    pub fn from_descriptor(descriptor: &str) -> Result<Self, ParseError> {
        let mut parser = TypeParser::new(descriptor);
        let field_type = parser.next()?;
        parser.expect_end()?;
        Ok(field_type)
    }

    /// Create the zero value for the given field.
    pub fn zero_value(&self) -> Value {
        match self {
            FieldType::Long => Value::Long(Long(0)),
            FieldType::Float => Value::Float(Float(0.0)),
            FieldType::Double => Value::Double(Double(0.0)),
            FieldType::Reference(_) | FieldType::Array(_) => Value::Reference(Reference(0)),
            _ => Value::Int(Int(0)),
        }
    }

    /// A field type might involve (at most 1) class names in the signature.
    ///
    /// For class loading we need to be aware of any class names in the type.
    pub fn class_name(&self) -> Option<String> {
        match self {
            FieldType::Reference(class_name) => Some(class_name.clone()),
            FieldType::Array(field_type) => field_type.class_name(),
            _ => None
        }
    }

    pub fn descriptor(&self) -> String {
        match self {
            FieldType::Boolean => "Z".to_string(),
            FieldType::Byte => "B".to_string(),
            FieldType::Char => "C".to_string(),
            FieldType::Short => "S".to_string(),
            FieldType::Int => "I".to_string(),
            FieldType::Long => "J".to_string(),
            FieldType::Float => "F".to_string(),
            FieldType::Double => "D".to_string(),
            FieldType::Reference(class_name) => format!("L{};", class_name.replace(".", "/")),
            FieldType::Array(component) => format!("[{}", component.descriptor())
        }
    }

    pub fn width(&self) -> usize {
        match self {
            FieldType::Boolean | FieldType::Byte => 1,
            FieldType::Char | FieldType::Short => 2,
            FieldType::Long | FieldType::Double => 8,
            _ => 4
        }
    }

    // Value to match for array component types.
    pub fn arr_comp(&self) -> usize {
        match self {
            FieldType::Boolean | FieldType::Byte => 0,
            FieldType::Char => 1,
            FieldType::Short => 2,
            FieldType::Int => 3,
            FieldType::Long => 4,
            FieldType::Float => 5,
            FieldType::Double => 6,
            FieldType::Reference(_) | FieldType::Array(_) => 7
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_descriptor_primitive() {
        assert_eq!(FieldType::from_descriptor("Z").unwrap(), FieldType::Boolean);
        assert_eq!(FieldType::from_descriptor("B").unwrap(), FieldType::Byte);
        assert_eq!(FieldType::from_descriptor("C").unwrap(), FieldType::Char);
        assert_eq!(FieldType::from_descriptor("S").unwrap(), FieldType::Short);
        assert_eq!(FieldType::from_descriptor("I").unwrap(), FieldType::Int);
        assert_eq!(FieldType::from_descriptor("J").unwrap(), FieldType::Long);
        assert_eq!(FieldType::from_descriptor("F").unwrap(), FieldType::Float);
        assert_eq!(FieldType::from_descriptor("D").unwrap(), FieldType::Double);
    }

    #[test]
    fn from_descriptor_class() {
        assert_eq!(FieldType::from_descriptor("Labc;").unwrap(), FieldType::Reference("abc".to_string()));
        assert_eq!(FieldType::from_descriptor("Ljava/thing;").unwrap(), FieldType::Reference("java/thing".to_string()));
    }

    #[test]
    fn from_descriptor_array() {
        assert_eq!(FieldType::from_descriptor("[Z").unwrap(), FieldType::Array(Box::new(FieldType::Boolean)));
        assert_eq!(FieldType::from_descriptor("[Ljava;").unwrap(), FieldType::Array(Box::new(FieldType::Reference("java".to_string()))));
        assert_eq!(FieldType::from_descriptor("[[Z").unwrap(), FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Boolean)))));
    }

    #[test]
    fn from_descriptor_errors() {
        assert!(FieldType::from_descriptor("").is_err());
        assert!(FieldType::from_descriptor("A").is_err());
        assert!(FieldType::from_descriptor("L").is_err());
        assert!(FieldType::from_descriptor("Labc").is_err());
        assert!(FieldType::from_descriptor("L;").is_err());
        assert!(FieldType::from_descriptor("[").is_err());
        assert!(FieldType::from_descriptor("[L").is_err());
        assert!(FieldType::from_descriptor("[L;").is_err());
    }

    #[test]
    fn zero_value() {
        assert_eq!(FieldType::Boolean.zero_value(), Value::Int(Int(0)));
        assert_eq!(FieldType::Byte.zero_value(), Value::Int(Int(0)));
        assert_eq!(FieldType::Char.zero_value(), Value::Int(Int(0)));
        assert_eq!(FieldType::Short.zero_value(), Value::Int(Int(0)));
        assert_eq!(FieldType::Int.zero_value(), Value::Int(Int(0)));
        assert_eq!(FieldType::Long.zero_value(), Value::Long(Long(0)));
        assert_eq!(FieldType::Float.zero_value(), Value::Float(Float(0.0)));
        assert_eq!(FieldType::Double.zero_value(), Value::Double(Double(0.0)));
        assert_eq!(FieldType::Reference("abc".to_string()).zero_value(), Value::Reference(Reference(0)));
        assert_eq!(FieldType::Array(Box::new(FieldType::Byte)).zero_value(), Value::Reference(Reference(0)));
    }
}