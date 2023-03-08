use std::sync::Arc;

use crate::java::{CategoryOne, MethodType, Value};
use crate::method_area;
use crate::native::{Args, Plugin};
use crate::runtime::Runtime;

type Function = Arc<dyn Fn(&Args) -> Option<Value> + Sync + Send>;

/// Some native method implementations require no state, so there's no need to create separate
/// internal types for those specific to their implementations.
struct StatelessPlugin {
    method: Method,
    function: Function,
}

pub struct Method {
    pub class: String,
    pub name: String,
    pub descriptor: MethodType,
}

/// Create a new plugin that simply delegates the given method to the function.
pub fn stateless(method: Method, function: Function) -> Box<dyn Plugin> {
    Box::new(StatelessPlugin { method, function }) as _
}

impl Plugin for StatelessPlugin {
    fn supports(&self, method: &method_area::Method) -> bool {
        let class = unsafe { method.class.as_ref().unwrap() };

        self.method.class.eq(&class.name) &&
            self.method.name.eq(&method.name) &&
            self.method.descriptor.eq(&method.descriptor)
    }

    fn call(&self, _: &method_area::Method, args: &Args) -> Option<Value> {
        (self.function)(args)
    }
}
