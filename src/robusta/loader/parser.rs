use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::class_file::{ClassAttribute, ClassFile, Code, CodeAttribute, const_pool, ExHandler, Field, LineNumber, LineNumberTable, MAGIC, Method, MethodAttribute, SourceFile, UnknownAttribute};
use crate::class_file::const_pool::{Class, Const, FieldRef, Integer, MethodRef, NameAndType, Utf8};

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
            const_pool: HashMap::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
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
        let skip_bytes = interfaces_count as usize * 2;
        self.read_length(skip_bytes)?;

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
            12 => Ok(Const::NameAndType(NameAndType {
                name: self.read_u16()?,
                descriptor: self.read_u16()?,
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    // #[test]
    // fn empty_main() {
    //     let mut file = File::open("./classes/EmptyMain.class").unwrap();
    //
    //     let class_file = parse(&mut file);
    //
    //     assert_eq!(class_file.minor_version, 0);
    //     assert_eq!(class_file.major_version, 63);
    //     assert_eq!(class_file.const_pool.len(), 14);
    //     assert_eq!(class_file.get_const_method(1), &MethodRef { class: 2, name_and_type: 3 });
    //     assert_eq!(class_file.get_const_class(2), &Class { name: 4 });
    //     assert_eq!(class_file.get_const_name_and_type(3), &NameAndType { name: 5, descriptor: 6 });
    //     assert_eq!(class_file.get_const_utf8(4), &Utf8 { bytes: "java/lang/Object".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(5), &Utf8 { bytes: "<init>".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(6), &Utf8 { bytes: "()V".as_bytes().into() });
    //     assert_eq!(class_file.get_const_class(7), &Class { name: 8 });
    //     assert_eq!(class_file.get_const_utf8(8), &Utf8 { bytes: "EmptyMain".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(9), &Utf8 { bytes: "Code".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(10), &Utf8 { bytes: "LineNumberTable".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(11), &Utf8 { bytes: "main".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(12), &Utf8 { bytes: "([Ljava/lang/String;)V".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(13), &Utf8 { bytes: "SourceFile".as_bytes().into() });
    //     assert_eq!(class_file.get_const_utf8(14), &Utf8 { bytes: "EmptyMain.java".as_bytes().into() });
    //     assert_eq!(class_file.access_flags, 0x21);
    //     assert_eq!(class_file.this_class, 7);
    //     assert_eq!(class_file.super_class, 2);
    //     assert_eq!(class_file.fields.len(), 0);
    //     assert_eq!(class_file.methods.len(), 2);
    //     assert_eq!(class_file.methods.get(0).unwrap(), &Method {
    //         access_flags: 0x1,
    //         name: 5,
    //         descriptor: 6,
    //         code: Some(Code {
    //             max_stack: 1,
    //             max_locals: 1,
    //             code: vec![0x2a, 0xb7, 0, 1, 0xb1],
    //         }),
    //     });
    //     assert_eq!(class_file.methods.get(1).unwrap(), &Method {
    //         access_flags: 0x9,
    //         name: 11,
    //         descriptor: 12,
    //         code: Some(Code {
    //             max_stack: 0,
    //             max_locals: 1,
    //             code: vec![0xb1],
    //         }),
    //     });
    // }
}