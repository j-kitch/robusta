use std::sync::Arc;

use crate::java::{MethodType, Value};
use crate::native::hash_code::hash_code_plugins;
use crate::native::robusta::robusta_plugins;
use crate::runtime::Runtime;

pub struct Plugins {
    plugins: Vec<Box<dyn Plugin>>,
}

unsafe impl Send for Plugins {}
unsafe impl Sync for Plugins {}

impl Plugins {
    pub fn new() -> Self {
        let mut plugins = Vec::new();
        plugins.append(&mut hash_code_plugins());
        plugins.append(&mut robusta_plugins());
        Plugins { plugins }
    }
}

impl Plugins {
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