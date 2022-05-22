use std::io;
use std::io::Error;
use crate::robusta::class_file::Const;

use super::Reader;

pub enum Attribute {
    Other { name: String, info: Vec<u8> }
}

pub struct ConstantValue {
    idx: u16,
}

pub struct Code {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<Handler>,
    attributes: Vec<Attribute>,
}

pub struct Handler {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl<R: io::BufRead> Reader<R> {
    pub fn read_constant_value(&mut self) -> Result<ConstantValue, Error> {
        let length = self.read_u32()?;
        self.expect(length == 2)?;

        let idx = self.read_u16()?;

        Ok(ConstantValue { idx })
    }

    pub fn read_code(&mut self, const_pool: &[Const]) -> Result<Code, Error> {
        let _ = self.read_u32()?;
        let max_stack = self.read_u16()?;
        let max_locals = self.read_u16()?;
        let code_length = self.read_u32()? as usize;
        let code = self.read_exact(code_length)?;
        let exception_table_length = self.read_u16()? as usize;
        let mut exception_table = Vec::with_capacity(exception_table_length);
        for _ in 0..exception_table_length {
            exception_table.push(Handler {
                start_pc: self.read_u16()?,
                end_pc: self.read_u16()?,
                handler_pc: self.read_u16()?,
                catch_type: self.read_u16()?,
            });
        }
        let attributes_length = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attributes_length);
        for _ in 0..attributes_length {
            attributes.push(self.read_attribute(const_pool)?);
        }
        Ok(Code {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}
