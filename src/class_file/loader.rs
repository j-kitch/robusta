use std::{io, mem};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};

use crate::class_file;
use crate::class_file::{ClassFile, Code, Const, Method};

pub struct Loader {
    reader: BufReader<File>,
    buffer: [u8; 8],
}

#[derive(Debug)]
pub struct LoadError(Box<dyn Error>);

impl LoadError {
    pub fn new<E: Error + Into<Box<dyn Error>>>(error: E) -> Self {
        LoadError(error.into())
    }

    pub fn simple(error: &str) -> Self {
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
pub struct SimpleError(String);

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for SimpleError {}

impl Loader {
    fn read_class_file(&mut self) -> Result<ClassFile, LoadError> {
        let magic = self.read_u32()?;
        if magic != class_file::MAGIC {
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
            methods: vec![],
        };

        class_file.minor_version = self.read_u16()?;
        class_file.major_version = self.read_u16()?;

        let const_pool_count = self.read_u16()?;
        let mut idx = 1;
        while idx < const_pool_count {
            let const_item = self.read_const()?;
            let size = const_item.size();
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
        if fields_count > 0 {
            return Err(LoadError::simple("Not handling fields yet"));
        }

        let methods_count = self.read_u16()?;
        for _ in 0..methods_count {
            let method = self.read_method(&class_file.const_pool)?;
            class_file.methods.push(method);
        }

        Ok(class_file)
    }

    fn read_const(&mut self) -> Result<Const, LoadError> {
        let tag = self.read_u8()?;
        match tag {
            1 => {
                let length = self.read_u16()?;
                let bytes = self.read_length(length as usize)?;
                Ok(Const::Utf8 { bytes })
            }
            7 => Ok(Const::Class {
                name: self.read_u16()?
            }),
            10 => Ok(Const::MethodRef {
                class: self.read_u16()?,
                name_and_type: self.read_u16()?,
            }),
            12 => Ok(Const::NameAndType {
                name: self.read_u16()?,
                descriptor: self.read_u16()?,
            }),
            _ => Err(LoadError::simple("Unknown tag"))
        }
    }

    fn read_method(&mut self, const_pool: &HashMap<u16, Const>) -> Result<Method, LoadError> {
        let mut method = Method {
            access_flags: 0,
            name: 0,
            descriptor: 0,
            code: None,
        };

        method.access_flags = self.read_u16()?;
        method.name = self.read_u16()?;
        method.descriptor = self.read_u16()?;

        let attribute_count = self.read_u16()?;
        for _ in 0..attribute_count {
            let name_idx = self.read_u16()?;
            let length = self.read_u32()?;

            let name = const_pool.get(&name_idx).ok_or(LoadError::simple("Expected Utf8"))?;
            let name = if let Const::Utf8 { bytes } = name {
                String::from_utf8(bytes.clone()).map_err(LoadError::new)
            } else {
                Err(LoadError::simple("Expected Utf8"))
            }?;

            if name.eq("Code") {
                method.code = Some(self.read_code(length as usize)?);
            } else {
                self.read_length(length as usize)?;
            }
        }

        Ok(method)
    }

    fn read_code(&mut self, length: usize) -> Result<Code, LoadError> {
        let mut code = Code {
            max_stack: 0,
            max_locals: 0,
            code: vec![],
        };

        code.max_stack = self.read_u16()?;
        code.max_locals = self.read_u16()?;

        let code_length = self.read_u32()?;
        code.code = self.read_length(code_length as usize)?;

        let skip_bytes = length - 2 - 2 - 4 - code_length as usize;
        self.read_length(skip_bytes)?;

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

    fn read_length(&mut self, length: usize) -> Result<Vec<u8>, LoadError> {
        let mut vec = vec![0; length];
        self.reader.read_exact(&mut vec).map_err(LoadError::new)?;
        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn empty_main() {
        let mut reader = BufReader::new(File::open(Path::new("./classes/EmptyMain.class")).unwrap());
        let mut loader = Loader { reader, buffer: [0; 8] };

        let class_file = loader.read_class_file().unwrap();

        assert_eq!(class_file.minor_version, 0);
        assert_eq!(class_file.major_version, 63);
        assert_eq!(class_file.const_pool.len(), 14);
        assert_eq!(class_file.const_pool.get(&1).unwrap(), &Const::MethodRef { class: 2, name_and_type: 3 });
        assert_eq!(class_file.const_pool.get(&2).unwrap(), &Const::Class { name: 4 });
        assert_eq!(class_file.const_pool.get(&3).unwrap(), &Const::NameAndType { name: 5, descriptor: 6 });
        assert_eq!(class_file.const_pool.get(&4).unwrap(), &Const::Utf8 { bytes: "java/lang/Object".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&5).unwrap(), &Const::Utf8 { bytes: "<init>".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&6).unwrap(), &Const::Utf8 { bytes: "()V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&7).unwrap(), &Const::Class { name: 8 });
        assert_eq!(class_file.const_pool.get(&8).unwrap(), &Const::Utf8 { bytes: "EmptyMain".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&9).unwrap(), &Const::Utf8 { bytes: "Code".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&10).unwrap(), &Const::Utf8 { bytes: "LineNumberTable".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&11).unwrap(), &Const::Utf8 { bytes: "main".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&12).unwrap(), &Const::Utf8 { bytes: "([Ljava/lang/String;)V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&13).unwrap(), &Const::Utf8 { bytes: "SourceFile".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&14).unwrap(), &Const::Utf8 { bytes: "EmptyMain.java".as_bytes().into() });
        assert_eq!(class_file.access_flags, 0x21);
        assert_eq!(class_file.this_class, 7);
        assert_eq!(class_file.super_class, 2);
        assert_eq!(class_file.methods.len(), 2);
        assert_eq!(class_file.methods.get(0).unwrap(), &Method {
            access_flags: 0x1,
            name: 5,
            descriptor: 6,
            code: Some(Code {
                max_stack: 1,
                max_locals: 1,
                code: vec![0x2a, 0xb7, 0, 1, 0xb1],
            }),
        });
        assert_eq!(class_file.methods.get(1).unwrap(), &Method {
            access_flags: 0x9,
            name: 11,
            descriptor: 12,
            code: Some(Code {
                max_stack: 0,
                max_locals: 1,
                code: vec![0xb1],
            }),
        });
    }
}