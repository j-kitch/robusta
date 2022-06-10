use std::io;
use std::io::Error;

use crate::robusta::class_file::Const;

use super::Reader;

pub enum Attribute {
    ConstantValue(ConstantValue),
    Code(Code),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    LocalVariableTypeTable(LocalVariableTypeTable),
    MethodParameters(MethodParameterTable),
    Signature(Signature),
    RuntimeVisibleAnnotations(RuntimeAnnotations),
    RuntimeInvisibleAnnotations(RuntimeAnnotations),
    RuntimeVisibleParameterAnnotations(RuntimeParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(RuntimeParameterAnnotations),
    Unknown(Unknown),
}

pub struct ConstantValue {
    pub idx: u16,
}

pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<Handler>,
    pub attributes: Vec<Attribute>,
}

pub struct Unknown {
    pub name: String,
    pub info: Vec<u8>,
}

pub struct Handler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub struct LineNumberTable {
    table: Vec<LineNumber>,
}

pub struct LineNumber {
    start_pc: u16,
    line_number: u16,
}

pub struct LocalVariableTable {
    table: Vec<LocalVariable>,
}

pub struct LocalVariable {
    start_pc: u16,
    length: u16,
    name_idx: u16,
    descriptor_idx: u16,
    idx: u16,
}

pub struct LocalVariableTypeTable {
    table: Vec<LocalVariableType>,
}

pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name_idx: u16,
    signature_idx: u16,
    idx: u16,
}

pub struct MethodParameterTable {
    table: Vec<MethodParameter>,
}

pub struct MethodParameter {
    name_idx: u16,
    access_flags: u16,
}

pub struct Signature {
    signature_idx: u16,
}

pub struct RuntimeAnnotations {
    annotations: Vec<Annotation>
}

pub struct RuntimeParameterAnnotations {
    parameters: Vec<Vec<Annotation>>
}

pub struct Annotation {
    type_idx: u16,
    value_pairs: Vec<ElementValuePair>,
}

pub struct ElementValuePair {
    name_idx: u16,
    value: ElementValue,
}

pub enum ElementValue {
    ConstValue { idx: u16 },
    EnumConstValue { type_name_idx: u16, const_name_idx: u16 },
    ClassInfo { idx: u16 },
    Annotation { annotation: Annotation },
    ArrayValue { values: Vec<ElementValue> },
}

impl<R: io::BufRead> Reader<R> {
    pub fn read_unknown(&mut self, name: &str) -> Result<Unknown, Error> {
        let length = self.read_u32()? as usize;
        let info = self.read_exact(length)?;
        Ok(Unknown { name: name.to_string(), info })
    }

    pub fn read_constant_value(&mut self) -> Result<ConstantValue, Error> {
        let length = self.read_u32()?;
        self.expect(length == 2)?;

        let idx = self.read_u16()?;

        Ok(ConstantValue { idx })
    }

    pub fn read_code(&mut self, const_pool: &[Const]) -> Result<Code, Error> {
        let _ = self.read_u32()?;
        let max_stack = self.read_u16()?;
        let max_locals = self.read_u16()?;
        let code_length = self.read_u32()? as usize;
        let code = self.read_exact(code_length)?;
        let exception_table_length = self.read_u16()? as usize;
        let mut exception_table = Vec::with_capacity(exception_table_length);
        for _ in 0..exception_table_length {
            exception_table.push(Handler {
                start_pc: self.read_u16()?,
                end_pc: self.read_u16()?,
                handler_pc: self.read_u16()?,
                catch_type: self.read_u16()?,
            });
        }
        let attributes_length = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attributes_length);
        for _ in 0..attributes_length {
            attributes.push(self.read_attribute(const_pool)?);
        }
        Ok(Code {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }

    pub fn read_line_number_table(&mut self) -> Result<LineNumberTable, Error> {
        self.read_u32()?;
        let length = self.read_u16()? as usize;
        let mut table = Vec::with_capacity(length);
        for _ in 0..length {
            table.push(LineNumber {
                start_pc: self.read_u16()?,
                line_number: self.read_u16()?,
            })
        }
        Ok(LineNumberTable { table })
    }

    pub fn read_local_variable_table(&mut self) -> Result<LocalVariableTable, Error> {
        self.read_u32()?;
        let length = self.read_u16()? as usize;
        let mut table = Vec::with_capacity(length);
        for _ in 0..length {
            table.push(LocalVariable {
                start_pc: self.read_u16()?,
                length: self.read_u16()?,
                name_idx: self.read_u16()?,
                descriptor_idx: self.read_u16()?,
                idx: self.read_u16()?,
            })
        }
        Ok(LocalVariableTable { table })
    }

    pub fn read_local_variable_type_table(&mut self) -> Result<LocalVariableTypeTable, Error> {
        self.read_u32()?;
        let length = self.read_u16()? as usize;
        let mut table = Vec::with_capacity(length);
        for _ in 0..length {
            table.push(LocalVariableType {
                start_pc: self.read_u16()?,
                length: self.read_u16()?,
                name_idx: self.read_u16()?,
                signature_idx: self.read_u16()?,
                idx: self.read_u16()?,
            })
        }
        Ok(LocalVariableTypeTable { table })
    }

    pub fn read_method_parameters(&mut self) -> Result<MethodParameterTable, Error> {
        self.read_u32()?;
        let length = self.read_u8()? as usize;
        let mut table = Vec::with_capacity(length);
        for _ in 0..length {
            table.push(MethodParameter {
                name_idx: self.read_u16()?,
                access_flags: self.read_u16()?,
            })
        }
        Ok(MethodParameterTable { table })
    }

    pub fn read_signature(&mut self) -> Result<Signature, Error> {
        self.read_u32()?;
        Ok(Signature { signature_idx: self.read_u16()? })
    }

    pub fn read_runtime_annotations(&mut self) -> Result<RuntimeAnnotations, Error> {
        self.read_u32()?;
        let num_annotations = self.read_u16()? as usize;
        let mut annotations = Vec::with_capacity(num_annotations);
        for _ in 0..num_annotations {
            annotations.push(self.read_annotation()?);
        }
        Ok(RuntimeAnnotations { annotations })
    }

    pub fn read_runtime_parameter_annotations(&mut self) -> Result<RuntimeParameterAnnotations, Error> {
        self.read_u32()?;
        let num_params = self.read_u8()? as usize;
        let mut parameters = Vec::with_capacity(num_params);
        for _ in 0..num_params {
            let num_annotations = self.read_u16()? as usize;
            let mut annotations = Vec::with_capacity(num_annotations);
            for _ in 0..num_annotations {
                annotations.push(self.read_annotation()?);
            }
            parameters.push(annotations);
        }
        Ok(RuntimeParameterAnnotations { parameters })
    }

    pub fn read_annotation(&mut self) -> Result<Annotation, Error> {
        let type_idx = self.read_u16()?;
        let num_pairs = self.read_u16()? as usize;
        let mut value_pairs = Vec::with_capacity(num_pairs);
        for _ in 0..num_pairs {
            value_pairs.push(self.read_element_value_pair()?);
        }
        Ok(Annotation { type_idx, value_pairs })
    }

    pub fn read_element_value_pair(&mut self) -> Result<ElementValuePair, Error> {
        let name_idx = self.read_u16()?;
        let value = self.read_element_value()?;
        Ok(ElementValuePair { name_idx, value })
    }

    pub fn read_element_value(&mut self) -> Result<ElementValue, Error> {
        let tag = self.read_u8()? as char;
        match tag {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                let idx = self.read_u16()?;
                Ok(ElementValue::ConstValue { idx })
            }
            'e' => {
                let type_name_idx = self.read_u16()?;
                let const_name_idx = self.read_u16()?;
                Ok(ElementValue::EnumConstValue { type_name_idx, const_name_idx })
            }
            'c' => Ok(ElementValue::ClassInfo { idx: self.read_u16()? }),
            '@' => Ok(ElementValue::Annotation { annotation: self.read_annotation()? }),
            '[' => {
                let num_values = self.read_u16()? as usize;
                let mut values = Vec::with_capacity(num_values);
                for _ in 0..num_values {
                    values.push(self.read_element_value()?);
                }
                Ok(ElementValue::ArrayValue { values })
            }
            _ => panic!("unknown element value tag {}", tag)
        }
    }
}
