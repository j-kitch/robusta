use std::sync::Arc;
use crate::java::Value;
use crate::native::plugin::{Args, Method, Plugin};
use crate::runtime::Runtime;

type Function = Arc<dyn Fn(Arc<Runtime>, Vec<Value>) -> Option<Value> + Sync + Send>;

struct SimplePlugin {
    method: Method,
    function: Function,
}

pub fn simple(method: Method, function: Function) -> Box<dyn Plugin> {
    Box::new(SimplePlugin { method, function }) as _
}

impl Plugin for SimplePlugin {
    fn supports(&self, method: &Method) -> bool {
        &self.method == method
    }

    fn call(&self, _: &Method, args: &Args) -> Option<Value> {
        (self.function)(args.runtime.clone(), args.params.clone())
    }
}