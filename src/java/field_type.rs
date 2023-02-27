use crate::java::ParseError;
use crate::java::type_parser::TypeParser;

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
    /// Parse the `FieldType` instance from a field descriptor.
    ///
    /// For further reference, see [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
    pub fn from_descriptor(descriptor: &str) -> Result<Self, ParseError> {
        let mut parser = TypeParser::new(descriptor);
        let field_type = parser.next()?;
        parser.expect_end()?;
        Ok(field_type)
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
}