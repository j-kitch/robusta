use crate::java::{Int, Value};
use crate::thread::Thread;

pub fn wide(thread: &mut Thread) {
    let frame = thread.stack.last_mut().unwrap();
    let opcode = frame.read_u8();

    match opcode {
        0x84 => {
            let index = frame.read_u16();
            let value2 = frame.read_i16() as i32;
            let value1 = frame.local_vars.load_value(index).int().0;

            let result = value1.overflowing_add(value2).0;

            frame.local_vars.store_value(index, Value::Int(Int(result)));
        }
        _ => panic!("Not implemented wide {:#02x}", opcode)
    }
}