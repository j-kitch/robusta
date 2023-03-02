use std::collections::HashMap;

pub mod loader;

/// The expected value at the start of a class file, identifying the class file format.
const MAGIC: u32 = 0xCAFE_BABE;

/// Static access flag.
pub const ACCESS_FLAG_STATIC: u16 = 0x0008;
pub const ACCESS_FLAG_NATIVE: u16 = 0x0100;

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
}

#[derive(Debug, PartialEq)]
/// An entry in the Class File's constant pool.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4).
pub enum Const {
    /// The `CONSTANT_Utf8_info` structure is used to represent constant string values.
    Utf8 {
        /// The modified UTF-8 string, for more details on this specific format, see
        /// [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4.7).
        bytes: Vec<u8>
    },
    /// The `CONSTANT_Integer_info` structure is used to represent an int constant.
    Integer {
        /// The int constant.
        int: i32
    },
    /// The `CONSTANT_Class_info` structure, used to represent a class or an interface.
    Class {
        /// An index into the `const_pool`, a valid `Const::Utf8`, representing a valid
        /// binary class or interface name encoded in internal form.
        name: u16
    },
    /// The `CONSTANT_String_info` structure, used to represent a string constant.
    String {
        /// An index into the `const_pool`, a valid `Const::Utf8`, representing a string
        /// constant.
        string: u16,
    },
    /// The `CONSTANT_Fieldref_info` structure, used to represent a field on a class.
    FieldRef {
        /// An index into the `const_pool`, a valid `Const::Class`, representing the class
        /// that this field is defined on.
        class: u16,
        /// An index into the `const_pool`, a valid `Const::NameAndType`, representing the
        /// name and field signature of this method.
        name_and_type: u16,
    },
    /// The `CONSTANT_Methodref_info` structure, used to represent a method on a class.
    MethodRef {
        /// An index into the `const_pool`, a valid `Const::Class`, representing the class
        /// that this method is defined on.
        class: u16,
        /// An index into the `const_pool`, a valid `Const::NameAndType`, representing the
        /// name and method signature of this method.
        name_and_type: u16,
    },
    /// The `CONSTANT_NameAndType_info` structure is used to represent a field or method,
    /// without indicating which class or interface type it belongs to:
    NameAndType {
        /// An index into the `const_pool`, a valid `Const::Utf8`, representing either the special
        /// method name `"<init>", or a valid unqualified name denoting a field or method.
        name: u16,
        /// An index into the `const_pool`, a valid `Const::Utf8`, representing a valid field or
        /// method type descriptor.
        descriptor: u16,
    },
}

impl Const {
    fn size(&self) -> u16 {
        match self {
            _ => 1
        }
    }

    pub fn runtime_pool_order(&self) -> usize {
        match self {
            Const::Integer { int: _ } => 0,
            Const::String { string: _ } => 1,
            Const::Class { name: _ } => 2,
            Const::FieldRef { class: _, name_and_type: _ } => 3,
            Const::MethodRef { class: _, name_and_type: _ } => 4,
            _ => 5
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
    /// The code of the method, if the method is not native or abstract.
    pub code: Option<Code>,
}

#[derive(Debug, PartialEq)]
/// The Code attribute for a Method.
pub struct Code {
    /// The max size of the frame's stack.
    pub max_stack: u16,
    /// The max size of the frame's local variables.
    pub max_locals: u16,
    /// The Java Virtual Machine code executed in this method.
    pub code: Vec<u8>,
}
