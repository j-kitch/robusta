use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use nohash_hasher::BuildNoHashHasher;

use crate::class_file::{ClassAttribute, ClassFile, Code, CodeAttribute, const_pool, ExHandler, Field, LineNumber, LineNumberTable, MAGIC, Method, MethodAttribute, SourceFile, UnknownAttribute};
use crate::class_file::const_pool::{Class, Const, Double, FieldRef, Float, Integer, InterfaceMethodRef, InvokeDynamic, Long, MethodHandle, MethodRef, MethodType, NameAndType, Utf8};

/// Parse a class file structure from a reader.
pub fn parse(reader: &mut dyn Read) -> ClassFile {
    let mut parser = Parser { reader, buffer: [0; 8] };
    parser.read_class_file().unwrap()
}

/// The internal representation of a parser.
struct Parser<'a> {
    reader: &'a mut dyn Read,
    buffer: [u8; 8],
}

impl<'a> Parser<'a> {
    /// Read the full class file from the underlying source.
    ///
    /// This method fully consumes the loaders contents.
    fn read_class_file(&mut self) -> Result<ClassFile, LoadError> {
        let magic = self.read_u32()?;
        if magic != MAGIC {
            return Err(LoadError::simple("Expected magic constant"));
        }

        let mut class_file = ClassFile {
            magic,
            minor_version: 0,
            major_version: 0,
            const_pool: HashMap::with_hasher(BuildNoHashHasher::default()),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };

        class_file.minor_version = self.read_u16()?;
        class_file.major_version = self.read_u16()?;

        let const_pool_count = self.read_u16()?;
        let mut idx = 1;
        while idx < const_pool_count {
            let const_item = self.read_const()?;
            let size = const_item.width();
            class_file.const_pool.insert(idx, const_item);
            idx += size;
        }

        class_file.access_flags = self.read_u16()?;
        class_file.this_class = self.read_u16()?;
        class_file.super_class = self.read_u16()?;

        let interfaces_count = self.read_u16()?;
        for _ in 0..interfaces_count {
            class_file.interfaces.push(self.read_u16()?);
        }

        let fields_count = self.read_u16()?;
        for _ in 0..fields_count {
            let field = self.read_field(&class_file)?;
            class_file.fields.push(field);
        }

        let methods_count = self.read_u16()?;
        for _ in 0..methods_count {
            let method = self.read_method(&class_file)?;
            class_file.methods.push(method);
        }

        let attributes_count = self.read_u16()?;
        for _ in 0..attributes_count {
            class_file.attributes.push(self.read_class_attribute(&class_file)?);
        }

        Ok(class_file)
    }

    fn read_class_attribute(&mut self, file: &ClassFile) -> Result<ClassAttribute, LoadError> {
        let name_idx = self.read_u16()?;
        let name = String::from_utf8(file.get_const_utf8(name_idx).bytes.clone()).unwrap();

        match name.as_str() {
            "SourceFile" => {
                let length = self.read_u32()?;
                assert_eq!(length, 2);
                Ok(ClassAttribute::SourceFile(SourceFile {
                    source_file: self.read_u16()?
                }))
            }
            _ => {
                let length = self.read_u32()?;
                let bytes = self.read_length(length as usize)?;
                Ok(ClassAttribute::Unknown(UnknownAttribute {
                    name_idx,
                    bytes,
                }))
            }
        }
    }

    fn read_method_attribute(&mut self, file: &ClassFile) -> Result<MethodAttribute, LoadError> {
        let name_idx = self.read_u16()?;
        let name = String::from_utf8(file.get_const_utf8(name_idx).bytes.clone()).unwrap();

        match name.as_str() {
            "Code" => {
                let _ = self.read_u32()?;
                Ok(MethodAttribute::Code(self.read_code(file)?))
            }
            _ => {
                let length = self.read_u32()?;
                let bytes = self.read_length(length as usize)?;
                Ok(MethodAttribute::Unknown(UnknownAttribute {
                    name_idx,
                    bytes,
                }))
            }
        }
    }

    fn read_code_attribute(&mut self, file: &ClassFile) -> Result<CodeAttribute, LoadError> {
        let name_idx = self.read_u16()?;
        let name = String::from_utf8(file.get_const_utf8(name_idx).bytes.clone()).unwrap();

        match name.as_str() {
            "LineNumberTable" => {
                let _ = self.read_u32()?;
                let line_number_table_len = self.read_u16()?;
                let mut line_number_table = Vec::new();
                for _ in 0..line_number_table_len {
                    line_number_table.push(LineNumber {
                        start_pc: self.read_u16()?,
                        line_number: self.read_u16()?,
                    });
                }
                Ok(CodeAttribute::LineNumberTable(LineNumberTable {
                    table: line_number_table,
                }))
            }
            _ => {
                let length = self.read_u32()?;
                let bytes = self.read_length(length as usize)?;
                Ok(CodeAttribute::Unknown(UnknownAttribute {
                    name_idx,
                    bytes,
                }))
            }
        }
    }

    fn read_const(&mut self) -> Result<Const, LoadError> {
        let tag = self.read_u8()?;
        match tag {
            1 => {
                let length = self.read_u16()?;
                let bytes = self.read_length(length as usize)?;
                Ok(Const::Utf8(Utf8 { bytes }))
            }
            3 => Ok(Const::Integer(Integer {
                int: self.read_i32()?,
            })),
            4 => Ok(Const::Float(Float {
                float: self.read_f32()?,
            })),
            5 => Ok(Const::Long(Long {
                long: self.read_i64()?,
            })),
            6 => Ok(Const::Double(Double {
                double: self.read_f64()?,
            })),
            7 => Ok(Const::Class(Class {
                name: self.read_u16()?
            })),
            8 => Ok(Const::String(const_pool::String {
                string: self.read_u16()?
            })),
            9 => Ok(Const::FieldRef(FieldRef {
                class: self.read_u16()?,
                name_and_type: self.read_u16()?,
            })),
            10 => Ok(Const::MethodRef(MethodRef {
                class: self.read_u16()?,
                name_and_type: self.read_u16()?,
            })),
            11 => Ok(Const::InterfaceMethodRef(InterfaceMethodRef {
                class: self.read_u16()?,
                name_and_type: self.read_u16()?,
            })),
            12 => Ok(Const::NameAndType(NameAndType {
                name: self.read_u16()?,
                descriptor: self.read_u16()?,
            })),
            15 => Ok(Const::MethodHandle(MethodHandle {
                reference_kind: self.read_u8()?,
                reference_idx: self.read_u16()?,
            })),
            16 => Ok(Const::MethodType(MethodType {
                descriptor: self.read_u16()?,
            })),
            18 => Ok(Const::InvokeDynamic(InvokeDynamic {
                bootstrap_method_attr: self.read_u16()?,
                name_and_type: self.read_u16()?,
            })),
            _ => Err(LoadError::simple(format!("unknown tag {}", tag).as_str()))
        }
    }

    fn read_field(&mut self, _: &ClassFile) -> Result<Field, LoadError> {
        let mut field = Field {
            access_flags: 0,
            name: 0,
            descriptor: 0,
        };

        field.access_flags = self.read_u16()?;
        field.name = self.read_u16()?;
        field.descriptor = self.read_u16()?;

        // Ignore attributes for now.
        let attribute_count = self.read_u16()?;
        for _ in 0..attribute_count {
            self.read_u16()?;
            let length = self.read_u32()? as usize;
            self.read_length(length)?;
        }

        Ok(field)
    }

    fn read_method(&mut self, class_file: &ClassFile) -> Result<Method, LoadError> {
        let mut method = Method {
            access_flags: 0,
            name: 0,
            descriptor: 0,
            attributes: vec![],
        };

        method.access_flags = self.read_u16()?;
        method.name = self.read_u16()?;
        method.descriptor = self.read_u16()?;

        let attribute_count = self.read_u16()?;
        for _ in 0..attribute_count {
            method.attributes.push(self.read_method_attribute(class_file)?);
        }

        Ok(method)
    }

    fn read_code(&mut self, file: &ClassFile) -> Result<Code, LoadError> {
        let mut code = Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![],
            ex_table: vec![],
            attributes: vec![],
        };

        code.max_stack = self.read_u16()?;
        code.max_locals = self.read_u16()?;

        let code_length = self.read_u32()?;
        code.code = self.read_length(code_length as usize)?;

        let ex_table_len = self.read_u16()?;
        for _ in 0..ex_table_len {
            code.ex_table.push(ExHandler {
                start_pc: self.read_u16()?,
                end_pc: self.read_u16()?,
                handler_pc: self.read_u16()?,
                catch_type: self.read_u16()?,
            });
        }

        let attributes_count = self.read_u16()?;
        for _ in 0..attributes_count {
            code.attributes.push(self.read_code_attribute(file)?);
        }

        Ok(code)
    }

    fn read_u8(&mut self) -> Result<u8, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..1])
            .map_err(LoadError::new)?;
        Ok(self.buffer[0])
    }

    fn read_u16(&mut self) -> Result<u16, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..2])
            .map_err(LoadError::new)?;
        let u16_slice: &[u8; 2] = &self.buffer[0..2].try_into()
            .map_err(LoadError::new)?;
        Ok(u16::from_be_bytes(*u16_slice))
    }

    fn read_u32(&mut self) -> Result<u32, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..4])
            .map_err(LoadError::new)?;
        let u32_slice: &[u8; 4] = &self.buffer[0..4].try_into()
            .map_err(LoadError::new)?;
        Ok(u32::from_be_bytes(*u32_slice))
    }

    fn read_i32(&mut self) -> Result<i32, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..4])
            .map_err(LoadError::new)?;
        let i32_slice: &[u8; 4] = &self.buffer[0..4].try_into()
            .map_err(LoadError::new)?;
        Ok(i32::from_be_bytes(*i32_slice))
    }

    fn read_f32(&mut self) -> Result<f32, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..4])
            .map_err(LoadError::new)?;
        let f32_slice: &[u8; 4] = &self.buffer[0..4].try_into()
            .map_err(LoadError::new)?;
        Ok(f32::from_be_bytes(*f32_slice))
    }

    fn read_i64(&mut self) -> Result<i64, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..8])
            .map_err(LoadError::new)?;
        let i64_slice: &[u8; 8] = &self.buffer[0..8].try_into()
            .map_err(LoadError::new)?;
        Ok(i64::from_be_bytes(*i64_slice))
    }

    fn read_f64(&mut self) -> Result<f64, LoadError> {
        self.reader.read_exact(&mut self.buffer[0..8])
            .map_err(LoadError::new)?;
        let f64_slice: &[u8; 8] = &self.buffer[0..8].try_into()
            .map_err(LoadError::new)?;
        Ok(f64::from_be_bytes(*f64_slice))
    }

    fn read_length(&mut self, length: usize) -> Result<Vec<u8>, LoadError> {
        let mut vec = vec![0; length];
        self.reader.read_exact(&mut vec).map_err(LoadError::new)?;
        Ok(vec)
    }
}


/// An error occurring during class file loading.
#[derive(Debug)]
pub struct LoadError(Box<dyn Error>);

impl LoadError {
    fn new<E: Error + Into<Box<dyn Error>>>(error: E) -> Self {
        LoadError(error.into())
    }

    fn simple(error: &str) -> Self {
        LoadError(Box::new(SimpleError(error.to_string())))
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for LoadError {}

#[derive(Debug)]
struct SimpleError(String);

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for SimpleError {}
