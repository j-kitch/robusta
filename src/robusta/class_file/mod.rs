use std::collections::HashMap;

use crate::class_file::const_pool::Const;

pub mod const_pool;

/// The expected value at the start of a class file, identifying the class file format.
pub const MAGIC: u32 = 0xCAFE_BABE;

/// Static access flag.
pub const ACCESS_FLAG_STATIC: u16 = 0x0008;
pub const ACCESS_FLAG_NATIVE: u16 = 0x0100;
pub const ACCESS_FLAG_SYNC: u16 = 0x1000;

/// The binary representation of a class file.
///
/// For further reference, see [the JVM spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html).
pub struct ClassFile {
    /// The `magic` item supplies the MAGIC number identifying the class file format; it has the value 0xCAFEBABE.
    ///
    /// While this value is a constant, we keep it in the class file for verification.
    pub magic: u32,
    /// The minor portion of the class file's version. `m` in version `M.m`.
    pub minor_version: u16,
    /// The major portion of the class file's version.  `M` in version `M.m`.
    pub major_version: u16,
    /// The `const_pool` is a table of structures representing various string constants, classes
    /// and interface names, and other constants that are referred to within the `ClassFile`
    /// structure.
    ///
    /// Each entry is indexed by a `u16` position, that might not be continuous based on the size
    /// of various constants, so we use a `HashMap` to represent the pool.
    pub const_pool: HashMap<u16, Const>,
    /// A mask of flags to denote access permissions to properties of this class or interface.
    pub access_flags: u16,
    /// The value of `this_class` must be a valid index into `const_pool`, that entry must be
    /// a valid `Const::Class` instance, representing this class.
    pub this_class: u16,
    /// The value of `super_class` may be zero, denoting a super class of `java.lang.Object`,
    /// or non-zero, a valid index into `const_pool`, which must be a `Const::Class` representing
    /// the super class of this type.
    pub super_class: u16,
    /// The fields supplied in this class file.
    pub fields: Vec<Field>,
    /// The methods supplied in the class file.
    pub methods: Vec<Method>,
    pub attributes: Vec<ClassAttribute>,
}

impl ClassFile {
    pub fn get_const_utf8(&self, index: u16) -> &const_pool::Utf8 {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::Utf8(utf8) => utf8,
            other => panic!("Expected const utf8, got {:?}", other)
        }
    }

    pub fn get_const_integer(&self, index: u16) -> &const_pool::Integer {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::Integer(integer) => integer,
            _ => panic!()
        }
    }

    pub fn get_const_class(&self, index: u16) -> &const_pool::Class {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::Class(class) => class,
            _ => panic!()
        }
    }

    pub fn get_const_string(&self, index: u16) -> &const_pool::String {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::String(string) => string,
            other => panic!("Expected const string, got {:?}", other)
        }
    }

    pub fn get_const_field(&self, index: u16) -> &const_pool::FieldRef {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::FieldRef(field) => field,
            _ => panic!()
        }
    }

    pub fn get_const_method(&self, index: u16) -> &const_pool::MethodRef {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::MethodRef(method) => method,
            _ => panic!()
        }
    }

    pub fn get_const_name_and_type(&self, index: u16) -> &const_pool::NameAndType {
        let con = self.const_pool.get(&index).unwrap();
        match con {
            Const::NameAndType(name_and_type) => name_and_type,
            _ => panic!()
        }
    }
}

#[derive(Debug, PartialEq)]
/// The `field_info` struct of a Class File.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5).
pub struct Field {
    /// A mask of flags denoting access permissions to and properties of this field.
    pub access_flags: u16,
    /// A valid index into `const_pool`, which must be a valid `Const::Utf8` value, the name of
    /// this field.
    pub name: u16,
    /// A valid index into `const_pool`, which must be a valid `Const::Utf8` value, the descriptor
    /// of this field.
    pub descriptor: u16,
}

#[derive(Debug, PartialEq)]
/// The `method_info` struct of a Class File.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.6).
pub struct Method {
    /// A mask of flags denoting access permissions to and properties of this method.
    pub access_flags: u16,
    /// A valid index into `const_pool`, which must be a valid `Const::Utf8` value, the name of
    /// this method.
    pub name: u16,
    /// A valid index into `const_pool`, which must be a valid `Const::Utf8` value, the descriptor
    /// of this method signature.
    pub descriptor: u16,
    pub attributes: Vec<MethodAttribute>,
}

impl Method {
    pub fn code(&self) -> Option<&Code> {
        for attr in &self.attributes {
            match attr {
                MethodAttribute::Code(code) => return Some(code),
                _ => {}
            }
        }
        return None;
    }
}

#[derive(Clone, Debug, PartialEq)]
/// The Code attribute for a Method.
pub struct Code {
    /// The max size of the frame's stack.
    pub max_stack: u16,
    /// The max size of the frame's local variables.
    pub max_locals: u16,
    /// The Java Virtual Machine code executed in this method.
    pub code: Vec<u8>,
    pub ex_table: Vec<ExHandler>,
    pub attributes: Vec<CodeAttribute>,
}

impl Code {
    pub fn line_number_table(&self) -> Option<&LineNumberTable> {
        self.attributes.iter().find_map(|a| {
            match a {
                CodeAttribute::LineNumberTable(lnt) => Some(lnt),
                _ => None
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassAttribute {
    SourceFile(SourceFile),
    Unknown(UnknownAttribute),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MethodAttribute {
    Code(Code),
    Unknown(UnknownAttribute),
}


#[derive(Clone, Debug, PartialEq)]
pub enum CodeAttribute {
    LineNumberTable(LineNumberTable),
    Unknown(UnknownAttribute),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceFile {
    pub source_file: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnknownAttribute {
    pub name_idx: u16,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineNumberTable {
    pub table: Vec<LineNumber>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}