use std::sync::Arc;

use tracing::info;

use crate::java::{CategoryOne, Value};
use crate::method_area::Method;
use crate::native::java_lang::java_lang_plugins;
use crate::native::robusta::robusta_plugins;

// use crate::runtime::Runtime;

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
        info!(name=&method.name, descriptor=method.descriptor.descriptor(), "Calling native method");
        // println!("Looking for {}.{}{}", method.class.as_str(), method.name.as_str(), method.descriptor.descriptor());
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
    pub runtime: Arc<crate::runtime2::Runtime>,
    pub params: Vec<CategoryOne>,
}

pub trait Plugin {
    fn supports(&self, method: &Method) -> bool;
    fn call(&self, method: &Method, args: &Args) -> Option<Value>;
}

