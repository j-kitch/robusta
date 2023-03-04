use std::sync::Arc;

use crate::java::{MethodType, Value};
use crate::native::{Args, Method, Plugin};
use crate::runtime::Runtime;

type Function = Arc<dyn Fn(Arc<Runtime>, Vec<Value>) -> Option<Value> + Sync + Send>;

/// Some native method implementations require no state, so there's no need to create separate
/// internal types for those specific to their implementations.
struct StatelessPlugin {
    method: Method,
    function: Function,
}

/// Create a new plugin that simply delegates the given method to the function.
pub fn stateless(method: Method, function: Function) -> Box<dyn Plugin> {
    Box::new(StatelessPlugin { method, function }) as _
}

impl Plugin for StatelessPlugin {
    fn supports(&self, method: &Method) -> bool {
        &self.method == method
    }

    fn call(&self, _: &Method, args: &Args) -> Option<Value> {
        (self.function)(args.runtime.clone(), args.params.clone())
    }
}
