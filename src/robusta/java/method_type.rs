use crate::java::{FieldType, ParseError};
use crate::java::type_parser::TypeParser;

/// A representation of the type of a method.
///
/// For more information, see
/// [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
#[derive(Debug, PartialEq)]
pub struct MethodType {
    /// The `parameters` of the method.
    pub parameters: Vec<FieldType>,
    /// The return type of the method, `void` is represented by `None`.
    pub returns: Option<FieldType>,
}

impl MethodType {
    /// Parse the `MethodType` from the method descriptor.
    ///
    /// For further reference, see [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.3).
    pub fn from_descriptor(descriptor: &str) -> Result<Self, ParseError> {
        let mut parser = TypeParser::new(descriptor);
        let mut method_type = MethodType { parameters: Vec::new(), returns: None };
        parser.expect_char('(')?;
        while parser.peek()? != ')' {
            let param = parser.next()?;
            method_type.parameters.push(param);
        }
        parser.expect_char(')')?;
        if parser.peek()? == 'V' {
            parser.expect_char('V')?;
        } else {
            method_type.returns = Some(parser.next()?);
        }
        parser.expect_end()?;
        Ok(method_type)
    }

    /// Get all the class names in this method signature for the purpose of class loading.
    pub fn class_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for param in self.parameters.iter() {
            if let Some(name) = param.class_name() {
                result.push(name);
            }
        }
        if let Some(name) = self.returns.as_ref().and_then(|ft| ft.class_name()) {
            result.push(name)
        }
        result
    }

    pub fn descriptor(&self) -> String {
        let mut descriptor = "(".to_string();
        for param in &self.parameters {
            descriptor.push_str(&param.descriptor());
        }
        descriptor.push(')');
        if let Some(value) = &self.returns {
            descriptor.push_str(&value.descriptor());
        } else {
            descriptor.push('V');
        }
        descriptor
    }
}

#[cfg(test)]
mod tests {
    use crate::java::FieldType::{Array, Boolean, Int, Long, Reference};

    use super::*;

    #[test]
    fn from_descriptor() {
        assert_eq!(MethodType::from_descriptor("()V").unwrap(), MethodType { parameters: vec![], returns: None });
        assert_eq!(MethodType::from_descriptor("(IJ)Z").unwrap(), MethodType { parameters: vec![Int, Long], returns: Some(Boolean) });
        assert_eq!(MethodType::from_descriptor("(ILabc;)Ldef;").unwrap(), MethodType {
            parameters: vec![
                Int,
                Reference("abc".to_string()),
            ],
            returns: Some(Reference("def".to_string())),
        });
        assert_eq!(MethodType::from_descriptor("([I)[La;").unwrap(), MethodType {
            parameters: vec![
                Array(Box::new(Int))
            ],
            returns: Some(Array(Box::new(Reference("a".to_string())))),
        });
    }
}