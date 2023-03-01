use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::class_file;
use crate::class_file::{ClassFile, Code, Const, Field, Method};

/// A class file loader parses a class file structure from a file.
pub struct Loader {
    /// The underlying source of the class file.
    reader: BufReader<File>,
    /// A buffer for reading primitive values 1-8 bytes long.
    buffer: [u8; 8],
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

impl Loader {
    pub fn new(path: &Path) -> Result<Self, LoadError> {
        let reader = BufReader::new(File::open(path).map_err(LoadError::new)?);
        Ok(Loader { reader, buffer: [0; 8] })
    }

    /// Read the full class file from the underlying source.
    ///
    /// This method fully consumes the loaders contents.
    pub fn read_class_file(&mut self) -> Result<ClassFile, LoadError> {
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
            fields: vec![],
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
        for _ in 0..fields_count {
            let field = self.read_field(&class_file.const_pool)?;
            class_file.fields.push(field);
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
            3 => Ok(Const::Integer {
                int: self.read_i32()?,
            }),
            7 => Ok(Const::Class {
                name: self.read_u16()?
            }),
            8 => Ok(Const::String {
                string: self.read_u16()?
            }),
            9 => Ok(Const::FieldRef {
                class: self.read_u16()?,
                name_and_type: self.read_u16()?,
            }),
            10 => Ok(Const::MethodRef {
                class: self.read_u16()?,
                name_and_type: self.read_u16()?,
            }),
            12 => Ok(Const::NameAndType {
                name: self.read_u16()?,
                descriptor: self.read_u16()?,
            }),
            _ => Err(LoadError::simple(format!("unknown tag {}", tag).as_str()))
        }
    }

    fn read_field(&mut self, const_pool: &HashMap<u16, Const>) -> Result<Field, LoadError> {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::class_file::Const::FieldRef;

    use super::*;

    #[test]
    fn empty_main() {
        let mut loader = Loader::new(Path::new("./classes/EmptyMain.class")).unwrap();

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
        assert_eq!(class_file.fields.len(), 0);
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

    #[test]
    fn print_constants() {
        let mut loader = Loader::new(Path::new("./classes/PrintConstants.class")).unwrap();

        let class_file = loader.read_class_file().unwrap();

        assert_eq!(class_file.minor_version, 0);
        assert_eq!(class_file.major_version, 63);
        assert_eq!(class_file.const_pool.len(), 26);
        assert_eq!(class_file.const_pool.get(&1).unwrap(), &Const::MethodRef { class: 2, name_and_type: 3 });
        assert_eq!(class_file.const_pool.get(&2).unwrap(), &Const::Class { name: 4 });
        assert_eq!(class_file.const_pool.get(&3).unwrap(), &Const::NameAndType { name: 5, descriptor: 6 });
        assert_eq!(class_file.const_pool.get(&4).unwrap(), &Const::Utf8 { bytes: "java/lang/Object".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&5).unwrap(), &Const::Utf8 { bytes: "<init>".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&6).unwrap(), &Const::Utf8 { bytes: "()V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&7).unwrap(), &Const::String { string: 8 });
        assert_eq!(class_file.const_pool.get(&8).unwrap(), &Const::Utf8 { bytes: "hello world".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&9).unwrap(), &Const::Integer { int: 542354326 });
        assert_eq!(class_file.const_pool.get(&10).unwrap(), &Const::MethodRef { class: 11, name_and_type: 12 });
        assert_eq!(class_file.const_pool.get(&11).unwrap(), &Const::Class { name: 13 });
        assert_eq!(class_file.const_pool.get(&12).unwrap(), &Const::NameAndType { name: 14, descriptor: 15 });
        assert_eq!(class_file.const_pool.get(&13).unwrap(), &Const::Utf8 { bytes: "Robusta".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&14).unwrap(), &Const::Utf8 { bytes: "println".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&15).unwrap(), &Const::Utf8 { bytes: "(I)V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&16).unwrap(), &Const::MethodRef { class: 11, name_and_type: 17 });
        assert_eq!(class_file.const_pool.get(&17).unwrap(), &Const::NameAndType { name: 14, descriptor: 18 });
        assert_eq!(class_file.const_pool.get(&18).unwrap(), &Const::Utf8 { bytes: "(Ljava/lang/String;)V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&19).unwrap(), &Const::Class { name: 20 });
        assert_eq!(class_file.const_pool.get(&20).unwrap(), &Const::Utf8 { bytes: "PrintConstants".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&21).unwrap(), &Const::Utf8 { bytes: "Code".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&22).unwrap(), &Const::Utf8 { bytes: "LineNumberTable".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&23).unwrap(), &Const::Utf8 { bytes: "main".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&24).unwrap(), &Const::Utf8 { bytes: "([Ljava/lang/String;)V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&25).unwrap(), &Const::Utf8 { bytes: "SourceFile".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&26).unwrap(), &Const::Utf8 { bytes: "PrintConstants.java".as_bytes().into() });
        assert_eq!(class_file.access_flags, 0x21);
        assert_eq!(class_file.this_class, 19);
        assert_eq!(class_file.super_class, 2);
        assert_eq!(class_file.fields.len(), 0);
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
            name: 23,
            descriptor: 24,
            code: Some(Code {
                max_stack: 1,
                max_locals: 3,
                code: vec![0x12, 0x7, 0x4c, 0x12, 0x9, 0x3d, 0x1c, 0xb8, 0, 0xa, 0x2b, 0xb8, 0x0, 0x10, 0xb1],
            }),
        });
    }

    #[test]
    fn java_lang_string() {
        let mut loader = Loader::new(Path::new("./classes/java/lang/String.class")).unwrap();

        let class_file = loader.read_class_file().unwrap();

        assert_eq!(class_file.minor_version, 0);
        assert_eq!(class_file.major_version, 52);
        assert_eq!(class_file.const_pool.len(), 20);
        assert_eq!(class_file.const_pool.get(&1).unwrap(), &Const::MethodRef { class: 4, name_and_type: 17 });
        assert_eq!(class_file.const_pool.get(&2).unwrap(), &Const::FieldRef { class: 3, name_and_type: 18 });
        assert_eq!(class_file.const_pool.get(&3).unwrap(), &Const::Class { name: 19 });
        assert_eq!(class_file.const_pool.get(&4).unwrap(), &Const::Class { name: 20 });
        assert_eq!(class_file.const_pool.get(&5).unwrap(), &Const::Utf8 { bytes: "chars".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&6).unwrap(), &Const::Utf8 { bytes: "[C".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&7).unwrap(), &Const::Utf8 { bytes: "<init>".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&8).unwrap(), &Const::Utf8 { bytes: "()V".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&9).unwrap(), &Const::Utf8 { bytes: "Code".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&10).unwrap(), &Const::Utf8 { bytes: "LineNumberTable".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&11).unwrap(), &Const::Utf8 { bytes: "getChars".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&12).unwrap(), &Const::Utf8 { bytes: "()[C".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&13).unwrap(), &Const::Utf8 { bytes: "fromUtf8".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&14).unwrap(), &Const::Utf8 { bytes: "([B)Ljava/lang/String;".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&15).unwrap(), &Const::Utf8 { bytes: "SourceFile".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&16).unwrap(), &Const::Utf8 { bytes: "String.java".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&17).unwrap(), &Const::NameAndType { name: 7, descriptor: 8 });
        assert_eq!(class_file.const_pool.get(&18).unwrap(), &Const::NameAndType { name: 5, descriptor: 6 });
        assert_eq!(class_file.const_pool.get(&19).unwrap(), &Const::Utf8 { bytes: "java/lang/String".as_bytes().into() });
        assert_eq!(class_file.const_pool.get(&20).unwrap(), &Const::Utf8 { bytes: "java/lang/Object".as_bytes().into() });
        assert_eq!(class_file.access_flags, 0x21);
        assert_eq!(class_file.this_class, 3);
        assert_eq!(class_file.super_class, 4);
        assert_eq!(class_file.fields.len(), 1);
        assert_eq!(class_file.fields.get(0).unwrap(), &Field {
            access_flags: 0x2,
            name: 5,
            descriptor: 6,
        });
        assert_eq!(class_file.methods.len(), 3);
        assert_eq!(class_file.methods.get(0).unwrap(), &Method {
            access_flags: 0x1,
            name: 7,
            descriptor: 8,
            code: Some(Code {
                max_stack: 1,
                max_locals: 1,
                code: vec![0x2a, 0xb7, 0, 1, 0xb1],
            }),
        });
        assert_eq!(class_file.methods.get(1).unwrap(), &Method {
            access_flags: 0x1,
            name: 11,
            descriptor: 12,
            code: Some(Code {
                max_stack: 1,
                max_locals: 1,
                code: vec![0x2a, 0xb4, 0, 0x2, 0xb0],
            }),
        });
        assert_eq!(class_file.methods.get(2).unwrap(), &Method {
            access_flags: 0x10a,
            name: 13,
            descriptor: 14,
            code: None,
        });
    }
}