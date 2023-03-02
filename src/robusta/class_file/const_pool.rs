#[derive(Debug, PartialEq)]
/// The `CONSTANT_Utf8_info` structure is used to represent constant string values.
pub struct Utf8 {
    /// The modified UTF-8 string, for more details on this specific format, see
    /// [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4.7).
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_Integer_info` structure is used to represent an int constant.
pub struct Integer {
    /// The int constant.
    pub int: i32,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_Class_info` structure, used to represent a class or an interface.
pub struct Class {
    /// An index into the `const_pool`, a valid `Const::Utf8`, representing a valid
    /// binary class or interface name encoded in internal form.
    pub name: u16,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_String_info` structure, used to represent a string constant.
pub struct String {
    /// An index into the `const_pool`, a valid `Const::Utf8`, representing a string
    /// constant.
    pub string: u16,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_Fieldref_info` structure, used to represent a field on a class.
pub struct FieldRef {
    /// An index into the `const_pool`, a valid `Const::Class`, representing the class
    /// that this field is defined on.
    pub class: u16,
    /// An index into the `const_pool`, a valid `Const::NameAndType`, representing the
    /// name and field signature of this method.
    pub name_and_type: u16,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_Methodref_info` structure, used to represent a method on a class.
pub struct MethodRef {
    /// An index into the `const_pool`, a valid `Const::Class`, representing the class
    /// that this method is defined on.
    pub class: u16,
    /// An index into the `const_pool`, a valid `Const::NameAndType`, representing the
    /// name and method signature of this method.
    pub name_and_type: u16,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_NameAndType_info` structure is used to represent a field or method,
/// without indicating which class or interface type it belongs to:
pub struct NameAndType {
    /// An index into the `const_pool`, a valid `Const::Utf8`, representing either the special
    /// method name `"<init>", or a valid unqualified name denoting a field or method.
    pub name: u16,
    /// An index into the `const_pool`, a valid `Const::Utf8`, representing a valid field or
    /// method type descriptor.
    pub descriptor: u16,
}

#[derive(Debug, PartialEq)]
/// An entry in the Class File's constant pool.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4).
pub enum Const {
    Utf8(Utf8),
    Integer(Integer),
    Class(Class),
    String(String),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    NameAndType(NameAndType),
}

impl Const {
    pub fn width(&self) -> u16 {
        match self {
            _ => 1
        }
    }

    /// When reading the constants in a class file, into the runtime constant pool, we need to
    /// make sure that we've read the class file constants in a correct order.
    ///
    /// For example, a `FieldRef` or `MethodRef` references a `Class`, so the `Class` must be
    /// inserted first.
    pub fn order(&self) -> usize {
        match self {
            Const::String(_) | Const::Integer(_) => 0,
            Const::Class(_) => 1,
            Const::FieldRef(_) | Const::MethodRef(_) => 2,
            _ => 3
        }
    }
}
