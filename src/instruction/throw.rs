use std::ops::Deref;
use crate::descriptor::Descriptor;
use crate::robusta::class::object::Handler;
use crate::thread::Thread;

pub fn a_throw(thread: &mut Thread) {
    loop {
        let mut runtime = thread.rt.borrow_mut();
        let mut current = thread.frames.current_mut();
        let pc = (current.pc - 1) as u16;

        let object_ref = current.op_stack.pop_ref();
        let obj = runtime.load_object(object_ref);
        let obj = obj.as_ref();
        let obj = obj.borrow_mut();

        let class = obj.obj().class.clone();

        let exception_table: &[Handler] = current.method.as_ref()
            .code.as_ref()
            .unwrap()
            .exception_table.as_ref();

        let handler = exception_table.iter()
            .find(|h| {
                let catch = h.catch_type.as_ref()
                    .map(|c| Descriptor::parse(c.as_ref().descriptor().as_str()))
                    .map_or(true, |ref d| class.is_instance_of(d));
                h.start_pc < pc
                    && pc <= h.end_pc
                    && catch
            });

        if handler.is_some() {
            current.pc = handler.unwrap().handler_pc as u32;
            return;
        } else {
            current.op_stack.push_ref(object_ref);
            thread.frames.pop();
        }
    }
}
