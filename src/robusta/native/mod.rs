use std::sync::Arc;

use crate::java::{MethodType, Value};
use crate::native::plugin::{Args, Method, Plugins};
use crate::runtime::heap::Array;
use crate::runtime::Runtime;

pub mod plugin;
mod hash_code;
mod robusta;
mod simple;

pub struct NativeMethods {
    plugins: Plugins,
}

impl NativeMethods {
    pub fn new() -> Self {
        NativeMethods {
            plugins: Plugins::new(),
        }
    }

    pub fn call(&self, method: &Method, args: &Args) -> Option<Value> {
        self.plugins.call(method, args)
    }
}

