use ErrorKind::Other;
use io::Error;
use std::io;
use std::io::ErrorKind;

const MAGIC_CODE: u32 = 0xCAFE_BABE;

pub struct ClassFile {
    pub version: Version,
    pub const_pool: Vec<Const>,
}

pub struct Version {
    pub minor: u16,
    pub major: u16,
}

#[derive(Debug, PartialEq)]
pub enum Const {
    Integer { int: i32 },
    Class { name_idx: u16 },
    String { string_idx: u16 },
    Field { class_idx: u16, descriptor_idx: u16 },
    Method { class_idx: u16, descriptor_idx: u16 },
    InterfaceMethod { class_idx: u16, descriptor_idx: u16 },
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
            3 => Ok(Const::Integer { int: self.read_i32()? }),
            7 => Ok(Const::Class { name_idx: self.read_u16()? }),
            8 => Ok(Const::String { string_idx: self.read_u16()? }),
            9 => Ok(Const::Field {
                class_idx: self.read_u16()?,
                descriptor_idx: self.read_u16()?,
            }),
            10 => Ok(Const::Method {
                class_idx: self.read_u16()?,
                descriptor_idx: self.read_u16()?,
            }),
            11 => Ok(Const::InterfaceMethod {
                class_idx: self.read_u16()?,
                descriptor_idx: self.read_u16()?,
            }),
            _ => Err(Error::new(Other, format!("Unknown Constant Pool tag {}", tag)))
        }
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
        Ok(ClassFile { version, const_pool })
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

    fn read_u64(&mut self) -> Result<u64, Error> {
        self.reader.read_exact(&mut self.u64_buffer[..])?;
        Ok(u64::from_be_bytes(self.u64_buffer))
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn read_const_integer() {
        let bytes = vec![3, 0, 0x10, 0x20, 0x30];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Integer { int: 0x10_2030 }, con);
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

        assert_eq!(Const::Field { class_idx: 1, descriptor_idx: 2 }, con);
    }

    #[test]
    fn read_const_method() {
        let bytes = vec![10, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::Method { class_idx: 1, descriptor_idx: 2 }, con);
    }

    #[test]
    fn read_const_interface_method() {
        let bytes = vec![11, 0, 1, 0, 2];
        let mut reader = Reader::new(&bytes[..]);

        let con = reader.read_const().unwrap();

        assert_eq!(Const::InterfaceMethod { class_idx: 1, descriptor_idx: 2 }, con);
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
        let bytes = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 40, 0, 50, 0, 1];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file().unwrap();

        assert_eq!(class_file.version.minor, 40);
        assert_eq!(class_file.version.major, 50);
        assert!(class_file.const_pool.is_empty());
    }

    #[test]
    fn read_class_file_maximal() {
        let bytes = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 40, 0, 50, 0, 2, 7, 0, 10];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file().unwrap();

        assert_eq!(class_file.version.minor, 40);
        assert_eq!(class_file.version.major, 50);
        assert_eq!(vec![Const::Class { name_idx: 10 }], class_file.const_pool);
    }
}
