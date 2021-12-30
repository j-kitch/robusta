use std::fs::File;
use std::io::Read;

pub struct Reader<'a> {
    file: &'a File,
    u32_buf: [u8; 4],
    u16_buf: [u8; 2],
    u8_buf: [u8; 1],
}

impl<'a> Reader<'a> {
    pub fn new(file: &'a File) -> Self {
        Reader { file, u32_buf: [0; 4], u16_buf: [0; 2], u8_buf: [0] }
    }

    fn read_u8(&mut self) -> u8 {
        self.file.read_exact(&mut self.u8_buf).unwrap();
        self.u8_buf[0]
    }

    fn read_u16(&mut self) -> u16 {
        self.file.read_exact(&mut self.u16_buf).unwrap();
        u16::from_be_bytes(self.u16_buf)
    }

    fn read_u32(&mut self) -> u32 {
        self.file.read_exact(&mut self.u32_buf).unwrap();
        u32::from_be_bytes(self.u32_buf)
    }

    fn read_i32(&mut self) -> i32 {
        self.file.read_exact(&mut self.u32_buf).unwrap();
        i32::from_be_bytes(self.u32_buf)
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = vec![0; len];
        self.file.read_exact(&mut bytes).unwrap();
        bytes
    }

    pub fn read_class_file(&mut self) -> ClassFile {
        self.read_u32();
        let minor_version = self.read_u16();
        let major_version = self.read_u16();
        let const_pool_len = self.read_u16();
        let const_pool: Vec<Const> = (1..const_pool_len).map(|_| self.read_const()).collect();
        let access_flags = self.read_u16();
        let this_class = self.read_u16();
        let super_class = self.read_u16();
        let interfaces_len = self.read_u16();
        let interfaces = (0..interfaces_len).map(|_| self.read_u16()).collect();
        let fields_len = self.read_u16();
        let fields = (0..fields_len).map(|_| self.read_field(&const_pool[..])).collect();
        let methods_len = self.read_u16();
        let methods = (0..methods_len).map(|_| self.read_method(&const_pool[..])).collect();
        let attributes_len = self.read_u16();
        let attributes = (0..attributes_len).map(|_| self.read_attribute(&const_pool[..])).collect();
        ClassFile { minor_version, major_version, const_pool, access_flags, this_class, super_class, interfaces, fields, methods, attributes }
    }

    fn read_const(&mut self) -> Const {
        let tag = self.read_u8();

        match tag {
            1 => {
                let length = self.read_u16();
                let bytes = self.read_bytes(length as usize);
                Const::Utf8(Utf8 { bytes })
            }
            3 => {
                let int = self.read_i32();
                Const::Int(Integer { int })
            }
            7 => {
                let name_idx = self.read_u16();
                Const::Class(Class { name_idx })
            }
            9 => {
                let class_idx = self.read_u16();
                let name_and_type_idx = self.read_u16();
                Const::FieldRef(FieldRef { class_idx, name_and_type_idx })
            }
            10 => {
                let class_idx = self.read_u16();
                let name_and_type_idx = self.read_u16();
                Const::MethodRef(MethodRef { class_idx, name_and_type_idx })
            }
            12 => {
                let name_idx = self.read_u16();
                let descriptor_idx = self.read_u16();
                Const::NameAndType(NameAndType { name_idx, descriptor_idx })
            }
            _ => {
                panic!("Unknown tag {}", tag);
            }
        }
    }

    fn read_field(&mut self, const_pool: &[Const]) -> Field {
        let access_flags = self.read_u16();
        let name_idx = self.read_u16();
        let descriptor_idx = self.read_u16();
        let attr_count = self.read_u16();
        let attributes = (0..attr_count).map(|_| self.read_attribute(const_pool)).collect();
        Field { access_flags, name_idx, descriptor_idx, attributes }
    }

    fn read_method(&mut self, const_pool: &[Const]) -> Method {
        let access_flags = self.read_u16();
        let name_idx = self.read_u16();
        let descriptor_idx = self.read_u16();
        let attr_count = self.read_u16();
        let attributes = (0..attr_count).map(|_| self.read_attribute(const_pool)).collect();
        Method { access_flags, name_idx, descriptor_idx, attributes }
    }

    fn read_attribute(&mut self, const_pool: &[Const]) -> Attribute {
        let name_idx = self.read_u16();
        let name = const_pool.get((name_idx - 1) as usize).unwrap().expect_utf8();
        let name = String::from_utf8(name.bytes.clone()).unwrap();

        match name.as_ref() {
            "Code" => {
                self.read_u32();
                let max_stack = self.read_u16();
                let max_locals = self.read_u16();
                let code_len = self.read_u32();
                let code = self.read_bytes(code_len as usize);
                let handler_len = self.read_u16();
                let ex_handlers = (0..handler_len).map(|_| {
                    let start_pc = self.read_u16();
                    let end_pc = self.read_u16();
                    let handler_pc = self.read_u16();
                    let catch_type = self.read_u16();
                    ExHandler { start_pc, end_pc, handler_pc, catch_type }
                }).collect();
                let attrs_len = self.read_u16();
                let attributes = (0..attrs_len).map(|_| self.read_attribute(const_pool)).collect();
                Attribute::Code(Code { max_stack, max_locals, code, ex_handlers, attributes })
            }
            _ => {
                let attr_len = self.read_u32();
                let bytes = self.read_bytes(attr_len as usize);
                Attribute::Unknown(UnknownAttr { _name_idx: name_idx, _bytes: bytes })
            }
        }
    }
}

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool: Vec<Const>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn get_const(&self, idx: u16) -> &Const {
        self.const_pool.get((idx - 1) as usize).unwrap()
    }
}

#[derive(Debug)]
pub struct Utf8 {
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct Class {
    pub name_idx: u16,
}

#[derive(Debug)]
pub struct Integer {
    pub int: i32,
}

#[derive(Debug)]
pub struct FieldRef {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

#[derive(Debug)]
pub struct MethodRef {
    pub class_idx: u16,
    pub name_and_type_idx: u16,
}

#[derive(Debug)]
pub struct NameAndType {
    pub name_idx: u16,
    pub descriptor_idx: u16,
}

#[derive(Debug)]
pub enum Const {
    Utf8(Utf8),
    Int(Integer),
    Class(Class),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    NameAndType(NameAndType),
}

impl Const {
    pub fn expect_utf8(&self) -> &Utf8 {
        match self {
            Const::Utf8(utf8) => utf8,
            _ => panic!("error")
        }
    }

    pub fn expect_class(&self) -> &Class {
        match self {
            Const::Class(class) => class,
            _ => panic!("error")
        }
    }

    pub fn expect_field_ref(&self) -> &FieldRef {
        match self {
            Const::FieldRef(field_ref) => field_ref,
            _ => panic!("error")
        }
    }

    pub fn expect_method_ref(&self) -> &MethodRef {
        match self {
            Const::MethodRef(method_ref) => method_ref,
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

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub enum Attribute {
    Unknown(UnknownAttr),
    Code(Code),
}

#[derive(Debug)]
pub struct UnknownAttr {
    _name_idx: u16,
    _bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub ex_handlers: Vec<ExHandler>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct ExHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}
