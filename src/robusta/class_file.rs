use ErrorKind::Other;
use io::Error;
use std::io;
use std::io::ErrorKind;

const MAGIC_CODE: u32 = 0xCAFE_BABE;

pub struct ClassFile {
    pub version: Version,
}

pub struct Version {
    pub minor: u16,
    pub major: u16,
}

struct ReadError {
    message: String
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

    pub fn read_class_file(&mut self) -> Result<ClassFile, Error> {
        let magic = self.read_u32()?;
        if magic != MAGIC_CODE {
            return Err(Error::new(Other, format!("Expected Magic Code 0xCAFEBABE, received {:#08X}", magic)))
        }

        let version = self.read_version()?;

        Ok(ClassFile { version })
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
        let bytes: Vec<u8> = vec![0, 10, 0, 20];
        let mut reader = Reader::new(&bytes[..]);

        let version = reader.read_version().unwrap();

        assert_eq!(version.minor, 10);
        assert_eq!(version.major, 20);
    }

    #[test]
    fn read_class_file_invalid_magic() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file();

        assert!(class_file.is_err());
    }

    #[test]
    fn read_class_file() {
        let bytes = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 40, 0, 50];
        let mut reader = Reader::new(&bytes[..]);

        let class_file = reader.read_class_file().unwrap();

        assert_eq!(class_file.version.minor, 40);
        assert_eq!(class_file.version.major, 50);
    }
}
