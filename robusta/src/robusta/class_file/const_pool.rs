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
/// The `CONSTANT_Float_info` structure is used to represent an int constant.
pub struct Float {
    /// The float constant.
    pub float: f32,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_Long_info` structure is used to represent an long constant.
pub struct Long {
    /// The long constant.
    pub long: i64,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_Double_info` structure is used to represent a double constant.
pub struct Double {
    /// The double constant.
    pub double: f64,
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
/// The `CONSTANT_InterfaceMethodref_info` structure, used to represent an interface method on a
/// class.
pub struct InterfaceMethodRef {
    /// An index into the `const_pool`, a valid `Const::Class`, representing the interface
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
/// The `CONSTANT_MethodHandle_info` structure.
pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_idx: u16,
}

#[derive(Debug, PartialEq)]
pub struct MethodType {
    pub descriptor: u16,
}

#[derive(Debug, PartialEq)]
/// The `CONSTANT_InvokeDynamic_info` structure.
pub struct InvokeDynamic {
    pub bootstrap_method_attr: u16,
    pub name_and_type: u16,
}

#[derive(Debug, PartialEq)]
/// An entry in the Class File's constant pool.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4).
pub enum Const {
    Utf8(Utf8),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    Class(Class),
    String(String),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    NameAndType(NameAndType),
    MethodHandle(MethodHandle),
    MethodType(MethodType),
    InvokeDynamic(InvokeDynamic),
}

impl Const {
    pub fn width(&self) -> u16 {
        match self {
            Const::Long(_) | Const::Double(_) => 2,
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
            Const::Class(_) => 0,
            Const::FieldRef(_) | Const::MethodRef(_) => 1,
            _ => 2
        }
    }
}
