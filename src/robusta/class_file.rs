use ErrorKind::Other;
use io::Error;
use std::io;
use std::io::ErrorKind;

const MAGIC_CODE: u32 = 0xCAFE_BABE;

pub struct ClassFile {
    pub version: Version,
    pub const_pool: Vec<Const>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

pub struct Version {
    pub minor: u16,
    pub major: u16,
}

#[derive(Debug, PartialEq)]
pub enum Const {
    Utf8 { utf8: String },
    Integer { int: i32 },
    Float { float: f32 },
    Long { long: i64 },
    Double { double: f64 },
    Class { name_idx: u16 },
    String { string_idx: u16 },
    Field { class_idx: u16, name_and_type_idx: u16 },
    Method { class_idx: u16, name_and_type_idx: u16 },
    InterfaceMethod { class_idx: u16, name_and_type_idx: u16 },
    NameAndType { name_idx: u16, descriptor_idx: u16 },
    MethodHandle { reference_kind: u8, reference_idx: u16 },
    MethodType { descriptor_idx: u16 },
    InvokeDynamic { bootstrap_idx: u16, name_and_type_idx: u16 },
}

pub struct Field {
    pub access_flags: u16,
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub attributes: Vec<Attribute>,
}

pub struct Method {
    pub access_flags: u16,
    pub name_idx: u16,
    pub descriptor_idx: u16,
    pub attributes: Vec<Attribute>,
}

pub enum Attribute {
    Other { name: String, info: Vec<u8> }
}

pub struct Reader<R: io::BufRead> {
    reader: R,
    u8_buffer: [u8; 1],
    u16_buffer: [u8; 2],
    u32_buffer: [u8; 4],
    u64_buffer: [u8; 8],
}

impl<R: io::BufRead> Reader<R> {
    pub fn new(reader: R) -> Self {
        Reader {
            reader,
            u8_buffer: [0],
            u16_buffer: [0; 2],
            u32_buffer: [0; 4],
            u64_buffer: [0; 8],
        }
    }

    pub fn read_version(&mut self) -> Result<Version, Error> {
        let minor = self.read_u16()?;
        let major = self.read_u16()?;
        Ok(Version { major, minor })
    }

    pub fn read_const(&mut self) -> Result<Const, Error> {
        let tag = self.read_u8()?;
        match tag {
            1 => {
                let length = self.read_u16()? as usize;
                let utf8 = self.read_utf8(length)?;
                Ok(Const::Utf8 { utf8 })
            }
            3 => Ok(Const::Integer { int: self.read_i32()? }),
            4 => Ok(Const::Float { float: self.read_f32()? }),
            5 => Ok(Const::Long { long: self.read_i64()? }),
            6 => Ok(Const::Double { double: self.read_f64()? }),
            7 => Ok(Const::Class { name_idx: self.read_u16()? }),
            8 => Ok(Const::String { string_idx: self.read_u16()? }),
            9 => Ok(Const::Field {
                class_idx: self.read_u16()?,
                name_and_type_idx: self.read_u16()?,
            }),
            10 => Ok(Const::Method {
                class_idx: self.read_u16()?,
                name_and_type_idx: self.read_u16()?,
            }),
            11 => Ok(Const::InterfaceMethod {
                class_idx: self.read_u16()?,
                name_and_type_idx: self.read_u16()?,
            }),
            12 => Ok(Const::NameAndType {
                name_idx: self.read_u16()?,
                descriptor_idx: self.read_u16()?,
            }),
            15 => Ok(Const::MethodHandle {
                reference_kind: self.read_u8()?,
                reference_idx: self.read_u16()?,
            }),
            16 => Ok(Const::MethodType {
                descriptor_idx: self.read_u16()?,
            }),
            18 => Ok(Const::InvokeDynamic {
                bootstrap_idx: self.read_u16()?,
                name_and_type_idx: self.read_u16()?,
            }),
            _ => Err(Error::new(Other, format!("Unknown Constant Pool tag {}", tag)))
        }
    }

    pub fn read_field(&mut self, const_pool: &[Const]) -> Result<Field, Error> {
        let access_flags = self.read_u16()?;
        let name_idx = self.read_u16()?;
        let descriptor_idx = self.read_u16()?;
        let attributes_count = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(self.read_attribute(const_pool)?);
        }

        Ok(Field { access_flags, name_idx, descriptor_idx, attributes })
    }

    pub fn read_method(&mut self, const_pool: &[Const]) -> Result<Method, Error> {
        let access_flags = self.read_u16()?;
        let name_idx = self.read_u16()?;
        let descriptor_idx = self.read_u16()?;
        let attributes_count = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            attributes.push(self.read_attribute(const_pool)?);
        }

        Ok(Method { access_flags, name_idx, descriptor_idx, attributes })
    }

    pub fn read_attribute(&mut self, const_pool: &[Const]) -> Result<Attribute, Error> {
        let name_idx = self.read_u16()? as usize;
        let name_const = &const_pool[name_idx - 1];
        let name = if let Const::Utf8 { utf8 } = name_const {
            utf8.clone()
        } else {
            panic!("Unexpected type")
        };

        let length = self.read_u32()? as usize;
        let info = self.read_exact(length)?;
        Ok(Attribute::Other { name, info })
    }

    pub fn read_class_file(&mut self) -> Result<ClassFile, Error> {
        let magic = self.read_u32()?;
        if magic != MAGIC_CODE {
            return Err(Error::new(Other, format!("Expected Magic Code 0xCAFEBABE, received {:#08X}", magic)));
        }

        let version = self.read_version()?;
        let const_pool_count = self.read_u16()? as usize;
        let mut const_pool = Vec::with_capacity(const_pool_count - 1);
        for _ in 1..const_pool_count {
            const_pool.push(self.read_const()?);
        }
        let access_flags = self.read_u16()?;
        let this_class = self.read_u16()?;
        let super_class = self.read_u16()?;

        let interfaces_count = self.read_u16()? as usize;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            interfaces.push(self.read_u16()?);
        }

        let fields_count = self.read_u16()? as usize;
        let mut fields = Vec::with_capacity(fields_count);
        for _ in 0..fields_count {
            fields.push(self.read_field(&const_pool[..])?);
        }

        let methods_count = self.read_u16()? as usize;
        let mut methods = Vec::with_capacity(methods_count);
        for _ in 0..fields_count {
            methods.push(self.read_method(&const_pool[..])?);
        }

        let attributes_count = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..fields_count {
            attributes.push(self.read_attribute(&const_pool[..])?);
        }

        Ok(ClassFile {
            version,
            const_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        self.reader.read_exact(&mut self.u8_buffer[..])?;
        Ok(self.u8_buffer[0])
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        self.reader.read_exact(&mut self.u16_buffer[..])?;
        Ok(u16::from_be_bytes(self.u16_buffer))
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        self.reader.read_exact(&mut self.u32_buffer[..])?;
        Ok(u32::from_be_bytes(self.u32_buffer))
    }

    fn read_i32(&mut self) -> Result<i32, Error> {
        self.reader.read_exact(&mut self.u32_buffer[..])?;
        Ok(i32::from_be_bytes(self.u32_buffer))
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        self.reader.read_exact(&mut self.u32_buffer[..])?;
        Ok(f32::from_be_bytes(self.u32_buffer))
    }

    fn read_u64(&mut self) -> Result<u64, Error> {
        self.reader.read_exact(&mut self.u64_buffer[..])?;
        Ok(u64::from_be_bytes(self.u64_buffer))
    }

    fn read_i64(&mut self) -> Result<i64, Error> {
        self.reader.read_exact(&mut self.u64_buffer[..])?;
        Ok(i64::from_be_bytes(self.u64_buffer))
    }

    fn read_f64(&mut self) -> Result<f64, Error> {
        self.reader.read_exact(&mut self.u64_buffer[..])?;
        Ok(f64::from_be_bytes(self.u64_buffer))
    }

    fn read_exact(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![0; length];
        self.reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    fn read_utf8(&mut self, length: usize) -> Result<String, Error> {
        let bytes = self.read_exact(length)?;
        String::from_utf8(bytes).map_err(|e| Error::new(Other, e))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_f32_infinity() {
        let bytes = vec![0x7F, 0x80, 0x00, 0x00];
        let mut reader = Reader::new(&bytes[..]);

        let f = reader.read_f32().unwrap();

        assert_eq!(f, f32::INFINITY);
    }

    #[test]
    fn read_f32_neg_infinity() {
        let bytes = vec![0xFF, 0x80, 0x00, 0x00];
        let mut reader = Reader::new(&bytes[..]);

        let f = reader.read_f32().unwrap();

        assert_eq!(f, -f32::INFINITY);
    }

    #[test]
    fn read_f32_nan() {
        let bytes = vec![0x7F, 0x80, 0, 1];
        let mut reader = Reader::new(&bytes[..]);

        let f = reader.read_f32().unwrap();

        assert!(f.is_nan());
    }

    #[test]
    fn read_version() {
        let bytes = vec![0, 10, 0, 20];
        let mut reader = Reader::new(&bytes[..]);

        let version = reader.read_version().unwrap();

        assert_eq!(version.minor, 10);
        assert_eq!(version.major, 20);
    }

    #[test]
    fn read_const_unknown_tag() {
        let bytes = vec![122];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const();

        assert!(con.is_err());
    }

    #[test]
    fn read_const_utf8() {
        let bytes = vec![1, 0, 3, 'a' as u8, 'b' as u8, 'c' as u8];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Utf8 { utf8: "abc".to_string() }, con);
    }

    #[test]
    fn read_const_integer() {
        let bytes = vec![3, 0, 0x10, 0x20, 0x30];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Integer { int: 0x10_2030 }, con);
    }

    #[test]
    fn read_const_float() {
        let bytes = vec![4, 10, 20, 30, 40];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Float { float: 0.0000000000000000000000000000000071316126 }, con);
    }

    #[test]
    fn read_const_long() {
        let bytes = vec![5, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Long { long: 72623859790382856 }, con);
    }

    #[test]
    fn read_const_double() {
        let bytes = vec![6, 64, 36, 117, 194, 143, 92, 40, 246];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Double { double: 10.23 }, con);
    }

    #[test]
    fn read_const_class() {
        let bytes = vec![7, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Class { name_idx: 2 }, con);
    }

    #[test]
    fn read_const_string() {
        let bytes = vec![8, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::String { string_idx: 2 }, con);
    }

    #[test]
    fn read_const_field() {
        let bytes = vec![9, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Field { class_idx: 1, name_and_type_idx: 2 }, con);
    }

    #[test]
    fn read_const_method() {
        let bytes = vec![10, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Method { class_idx: 1, name_and_type_idx: 2 }, con);
    }

    #[test]
    fn read_const_interface_method() {
        let bytes = vec![11, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::InterfaceMethod { class_idx: 1, name_and_type_idx: 2 }, con);
    }

    #[test]
    fn read_const_name_and_type() {
        let bytes = vec![12, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::NameAndType { name_idx: 1, descriptor_idx: 2 }, con);
    }

    #[test]
    fn read_const_method_handle() {
        let bytes = vec![15, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::MethodHandle { reference_kind: 1, reference_idx: 2 }, con);
    }

    #[test]
    fn read_const_method_type() {
        let bytes = vec![16, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::MethodType { descriptor_idx: 2 }, con);
    }

    #[test]
    fn read_const_invoke_dynamic() {
        let bytes = vec![18, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::InvokeDynamic { bootstrap_idx: 1, name_and_type_idx: 2 }, con);
    }

    #[test]
    fn read_class_file_invalid_magic() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file();

        assert!(class_file.is_err());
    }

    #[test]
    fn read_class_file_minimal() {
        let bytes = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 40, 0, 50, 0, 1, 0, 1, 0, 2, 0, 3, 0, 0];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file().unwrap();

        assert_eq!(class_file.version.minor, 40);
        assert_eq!(class_file.version.major, 50);
        assert!(class_file.const_pool.is_empty());
        assert_eq!(class_file.access_flags, 1);
        assert_eq!(class_file.this_class, 2);
        assert_eq!(class_file.super_class, 3);
        assert!(class_file.interfaces.is_empty());
    }

    #[test]
    fn read_class_file_maximal() {
        let bytes = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 40, 0, 50, 0, 2, 7, 0, 10, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0, 2, 0xAA, 0xAB, 0xBA, 0xBB];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file().unwrap();

        assert_eq!(class_file.version.minor, 40);
        assert_eq!(class_file.version.major, 50);
        assert_eq!(vec![Const::Class { name_idx: 10 }], class_file.const_pool);
        assert_eq!(class_file.access_flags, 0xF0F1);
        assert_eq!(class_file.this_class, 0xF2F3);
        assert_eq!(class_file.super_class, 0xF4F5);
        assert_eq!(class_file.interfaces, vec![0xAAAB, 0xBABB]);
    }
}
