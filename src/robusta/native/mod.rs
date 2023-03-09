use std::sync::Arc;

use tracing::debug;

use crate::java::Value;
use crate::log;
use crate::method_area::Method;
use crate::native::java_lang::java_lang_plugins;
use crate::native::robusta::robusta_plugins;
use crate::thread::Thread;

mod robusta;
mod stateless;
mod java_lang;

pub struct NativeMethods {
    plugins: Vec<Box<dyn Plugin>>,
}

unsafe impl Send for NativeMethods {}

unsafe impl Sync for NativeMethods {}

impl NativeMethods {
    pub fn new() -> Self {
        let mut plugins = Vec::new();
        plugins.append(&mut robusta_plugins());
        plugins.append(&mut java_lang_plugins());
        NativeMethods { plugins }
    }

    pub fn call(&self, method: &Method, args: &Args) -> Option<Value> {
        let class = unsafe { method.class.as_ref().unwrap() };
        debug!(
            target: log::THREAD,
            method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()),
            "Invoking native method"
        );
        let plugin = self.plugins.iter()
            .find(|p| p.supports(method))
            .unwrap();
        let result = plugin.call(method, args);
        // println!("Return");
        return result;
    }
}

// #[derive(PartialEq, Eq, Hash)]
// pub struct Method {
//     pub class: String,
//     pub name: String,
//     pub descriptor: MethodType,
// }

pub struct Args {
    pub thread: *const Thread,
    pub runtime: Arc<crate::runtime::Runtime>,
    pub params: Vec<Value>,
}

pub trait Plugin {
    fn supports(&self, method: &Method) -> bool;
    fn call(&self, method: &Method, args: &Args) -> Option<Value>;
}

