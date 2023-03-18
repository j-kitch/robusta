use std::sync::Arc;

use tracing::debug;

use crate::java::{Reference, Value};
use crate::log;
use crate::method_area::Method;
use crate::native::java_lang::java_lang_plugins;
use crate::native::java_security::java_security_plugins;
use crate::native::robusta::robusta_plugins;
use crate::native::system::system_plugins;
use crate::thread::Thread;

mod robusta;
mod stateless;
mod java_lang;
mod java_security;
mod system;

pub struct NativeMethods {
    plugins: Vec<Arc<dyn Plugin>>,
}

unsafe impl Send for NativeMethods {}

unsafe impl Sync for NativeMethods {}

impl NativeMethods {
    pub fn new() -> Self {
        let mut plugins = Vec::new();
        plugins.append(&mut robusta_plugins());
        plugins.append(&mut java_lang_plugins());
        plugins.append(&mut java_security_plugins());
        plugins.append(&mut system_plugins());
        NativeMethods { plugins }
    }

    pub fn find(&self, method: &Method) -> Option<Arc<dyn Plugin>> {
        let class = unsafe { method.class.as_ref().unwrap() };
        debug!(
            target: log::THREAD,
            method=format!("{}.{}{}", class.name.as_str(), method.name.as_str(), method.descriptor.descriptor()),
            "Invoking native method"
        );
        let plugin = self.plugins.iter()
            .find(|p| p.supports(method));

        plugin.map(|p| p.clone())
    }
}

pub struct Args {
    pub thread: *const Thread,
    pub runtime: Arc<crate::runtime::Runtime>,
    pub params: Vec<Value>,
}

impl Args {
    /// TODO: Not used yet.
    pub fn add_local(&self, reference: Reference) {
        let thread = unsafe { self.thread.cast_mut().as_mut().unwrap() };
        let frame = thread.stack.last_mut().unwrap();
        frame.native_roots.insert(reference);
    }

    pub fn enter_safe(&self) {
        let thread = unsafe { self.thread.cast_mut().as_mut().unwrap() };
        thread.safe.enter();
    }

    pub fn exit_safe(&self) {
        let thread = unsafe { self.thread.cast_mut().as_mut().unwrap() };
        thread.safe.exit();
    }
}

pub trait Plugin {
    fn supports(&self, method: &Method) -> bool;
    fn call(&self, method: &Method, args: &Args) -> Option<Value>;
}

