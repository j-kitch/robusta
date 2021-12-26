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
        let const_pool = (1..const_pool_len).map(|_| self.read_const()).collect();
        let access_flags = self.read_u16();
        let this_class = self.read_u16();
        let super_class = self.read_u16();
        let interfaces_len = self.read_u16();
        let interfaces = (0..interfaces_len).map(|_| self.read_u16()).collect();
        let fields_len = self.read_u16();
        if fields_len > 0 {
            panic!("Not implemented fields");
        }
        let methods_len = self.read_u16();
        let methods = (0..methods_len).map(|_| self.read_method()).collect();
        let attributes_len = self.read_u16();
        let attributes = (0..attributes_len).map(|_| self.read_attribute()).collect();
        ClassFile { minor_version, major_version, const_pool, access_flags, this_class, super_class, interfaces, methods, attributes }
    }

    fn read_const(&mut self) -> Const {
        let tag = self.read_u8();

        match tag {
            1 => {
                let length = self.read_u16();
                let bytes = self.read_bytes(length as usize);
                Const::Utf8(Utf8 { bytes })
            }
            7 => {
                let name_idx = self.read_u16();
                Const::Class(Class { name_idx })
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

    fn read_method(&mut self) -> Method {
        let access_flags = self.read_u16();
        let name_idx = self.read_u16();
        let descriptor_idx = self.read_u16();
        let attr_count = self.read_u16();
        let attributes = (0..attr_count).map(|_| self.read_attribute()).collect();
        Method { access_flags, name_idx, descriptor_idx, attributes }
    }

    fn read_attribute(&mut self) -> Attribute {
        let name_idx = self.read_u16();
        let attr_len = self.read_u32();
        let bytes = self.read_bytes(attr_len as usize);
        Attribute { name_idx, bytes }
    }
}

#[derive(Debug)]
pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    const_pool: Vec<Const>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct Utf8 {
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct Class {
    name_idx: u16,
}

#[derive(Debug)]
struct MethodRef {
    class_idx: u16,
    name_and_type_idx: u16,
}

#[derive(Debug)]
struct NameAndType {
    name_idx: u16,
    descriptor_idx: u16,
}

#[derive(Debug)]
enum Const {
    Utf8(Utf8),
    Class(Class),
    MethodRef(MethodRef),
    NameAndType(NameAndType),
}

#[derive(Debug)]
struct Method {
    access_flags: u16,
    name_idx: u16,
    descriptor_idx: u16,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct Attribute {
    name_idx: u16,
    bytes: Vec<u8>,
}
