use crate::descriptor::MethodDescriptor;
use crate::heap::Value;
use crate::native::NativePlugin;
use crate::runtime::Runtime;

// Many implementations of native plugins are just static functions that require
// no shared state.
pub struct Static {
    class: String,
    name: String,
    descriptor: MethodDescriptor,
    function: NativeFunction,
}

type NativeFunction = fn(runtime: &mut Runtime, args: Vec<Value>) -> Option<Value>;

impl Static {
    pub fn new(class: &str, name: &str, desc: MethodDescriptor, func: NativeFunction) -> Self {
        Static {
            class: class.to_string(),
            name: name.to_string(),
            descriptor: desc,
            function: func,
        }
    }
}

impl NativePlugin for Static {
    fn supports(&self, class: &str, name: &str, desc: &MethodDescriptor) -> bool {
        self.class.eq(class) && self.name.eq(name) && self.descriptor.eq(desc)
    }

    fn invoke(&mut self, runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
        (self.function)(runtime, args)
    }
}
