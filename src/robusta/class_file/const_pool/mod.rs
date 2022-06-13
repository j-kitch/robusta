pub enum Const {
    Utf8(Utf8),
    Integer(Integer),
    Float(Float),
    Long(Long),
    Double(Double),
    Class(Class),
    String(String),
    Field(Field),
    Method(Method),
    InterfaceMethod(InterfaceMethod),
    NameAndType(NameAndType),
    MethodHandle(MethodHandle),
    MethodDescriptor(MethodType),
    InvokeDynamic(InvokeDynamic),
}

impl Const {
    pub fn expect_utf8(&self) -> &Utf8 {
        match self {
            Const::Utf8(utf8) => utf8,
            _ => panic!("error")
        }
    }

    pub fn expect_integer(&self) -> &Integer {
        match self {
            Const::Integer(integer) => integer,
            _ => panic!("error")
        }
    }

    pub fn expect_float(&self) -> &Float {
        match self {
            Const::Float(float) => float,
            _ => panic!("error")
        }
    }

    pub fn expect_long(&self) -> &Long {
        match self {
            Const::Long(long) => long,
            _ => panic!("error")
        }
    }

    pub fn expect_double(&self) -> &Double {
        match self {
            Const::Double(double) => double,
            _ => panic!("error")
        }
    }

    pub fn expect_class(&self) -> &Class {
        match self {
            Const::Class(class) => class,
            _ => panic!("error")
        }
    }

    pub fn expect_string(&self) -> &String {
        match self {
            Const::String(string) => string,
            _ => panic!("error")
        }
    }

    pub fn expect_field(&self) -> &Field {
        match self {
            Const::Field(field) => field,
            _ => panic!("error")
        }
    }

    pub fn expect_method(&self) -> &Method {
        match self {
            Const::Method(method) => method,
            _ => panic!("error")
        }
    }

    pub fn expect_name_and_type(&self) -> &NameAndType {
        match self {
            Const::NameAndType(name_and_type) => name_and_type,
            _ => panic!("error")
        }
    }
}

pub struct Utf8 {
    pub utf8: std::string::String,
}

pub struct Integer {
    pub int: i32,
}

pub struct Float {
    pub float: f32,
}

pub struct Long {
    pub long: i64,
}

pub struct Double {
    pub double: f64,
}

pub struct Class {
    pub name_idx: u16,
}

pub struct String {
    pub string_idx: u16,
}

pub struct Field {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

pub struct Method {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

pub struct InterfaceMethod {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

pub struct NameAndType {
    pub name_idx: u16,
    pub descriptor_idx: u16,
}

pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_idx: u16,
}

pub struct MethodType {
    pub descriptor_idx: u16,
}

pub struct InvokeDynamic {
    pub bootstrap_idx: u16,
    pub name_and_type_idx: u16,
}
