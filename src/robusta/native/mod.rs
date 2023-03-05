use std::sync::Arc;

use crate::java::{MethodType, Value};
use crate::native::hash_code::hash_code_plugins;
use crate::native::robusta::robusta_plugins;
use crate::native::java_lang::java_lang_plugins;
use crate::runtime::Runtime;

mod hash_code;
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
        plugins.append(&mut hash_code_plugins());
        plugins.append(&mut robusta_plugins());
        plugins.append(&mut java_lang_plugins());
        NativeMethods { plugins }
    }

    pub fn call(&self, method: &Method, args: &Args) -> Option<Value> {
        let plugin = self.plugins.iter()
            .find(|p| p.supports(method))
            .unwrap();
        plugin.call(method, args)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Method {
    pub class: String,
    pub name: String,
    pub descriptor: MethodType,
}

pub struct Args {
    pub runtime: Arc<Runtime>,
    pub params: Vec<Value>,
}

pub trait Plugin {
    fn supports(&self, method: &Method) -> bool;
    fn call(&self, method: &Method, args: &Args) -> Option<Value>;
}

