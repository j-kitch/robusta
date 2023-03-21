use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use nohash_hasher::BuildNoHashHasher;

use parking_lot::Condvar;
use parking_lot::lock_api::Mutex;
use tracing::{debug, trace};

use crate::collection::wait::ThreadWait;
use crate::heap::sync::Synchronized;
use crate::instruction::instruction;
use crate::java::{CategoryOne, MethodType, Reference, Value};
use crate::log;
use crate::method_area::{ObjectClass, Method};
use crate::method_area::const_pool::{ConstPool, MethodKey};
use crate::native::{Args, Plugin};
use crate::runtime::Runtime;

pub struct Safe {
    name: String,
    state: parking_lot::Mutex<(bool, bool)>,
    wait: Condvar,
}

impl Safe {
    pub fn new(name: String) -> Self {
        Safe {
            name,
            state: Mutex::new((false, false)),
            wait: Condvar::new(),
        }
    }

    /// Let GC know that we are ready to start GC!
    pub fn enter(&self) {
        // trace!("enter safe");
        let mut lock = self.state.lock();
        lock.0 = true;
        self.wait.notify_all();
        // trace!("entered safe");
    }

    pub fn start_gc(&self) {
        let mut lock = self.state.lock();
        // Wait for thread to become safe.
        while !lock.0 {
            self.wait.wait_while(&mut lock, |(thread, _)| {
                !*thread
            });
        }
        lock.1 = true;
        debug!(target: log::GC, "Stopped thread {}", self.name.as_str());
    }

    /// Wait for GC to end.
    pub fn exit(&self) {
        // trace!("exiting safe");
        let mut lock = self.state.lock();
        while lock.1 {
            self.wait.wait_while(&mut lock, |(_, gc)| {
                *gc
            });
        }
        lock.0 = false;
        // trace!("exited safe");
    }

    pub fn end_gc(&self) {
        let mut lock = self.state.lock();
        lock.1 = false;
        self.wait.notify_all();
        trace!("Started thread {}", self.name.as_str());
    }

    pub fn safe_region(&self) {
        self.enter();
        self.exit();
    }
}

/// A single Java thread in the running program.
pub struct Thread {
    pub name: String,
    pub reference: Option<Reference>,
    pub locks: HashMap<u32, Synchronized, BuildNoHashHasher<u32>>,
    pub safe: Safe,
    /// A reference to the common runtime areas that are shared across one instance of a
    /// running program.
    pub runtime: Arc<Runtime>,
    /// The java virtual machine stack in this thread.
    ///
    /// The last frame on the stack is the currently active frame of the thread.
    pub stack: Vec<Frame>,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub fn enter_monitor(&mut self, object_ref: Reference) {
        if self.locks.contains_key(&object_ref.0) {
            let sync = self.locks.get_mut(&object_ref.0).unwrap();
            sync.enter();
        } else {
            let object = self.runtime.heap.get_object(object_ref);
            let header = unsafe { object.header.as_ref().unwrap() };
            self.safe.enter();
            let sync = header.lock.lock();
            self.safe.exit();
            self.locks.insert(object_ref.0, sync);
        }
    }

    pub fn exit_monitor(&mut self, object_ref: Reference) {
        let sync = self.locks.get_mut(&object_ref.0).unwrap();
        let should_remove = sync.exit();
        if should_remove {
            let sync = self.locks.remove(&object_ref.0).unwrap();
            drop(sync);
        }
    }

    pub fn as_mut<'a>(self: &'a Arc<Self>) -> &'a mut Thread {
        unsafe {
            let thread = self.as_ref() as *const Thread;
            thread.cast_mut().as_mut().unwrap()
        }
    }

    pub fn find_native(&self, method: &Method) -> Option<Arc<dyn Plugin>> {
        self.runtime.native.find(method)
    }

    /// A native method needs to be able to invoke the thread stack again to get a result.
    pub fn native_invoke(&mut self, class: *const ObjectClass, method: *const Method, args: Vec<Value>) -> (Option<Value>, Option<Value>) {
        let class = unsafe { class.as_ref().unwrap() };
        let method2 = unsafe { method.as_ref().unwrap() };
        let has_return = unsafe { method.as_ref().unwrap().descriptor.returns.is_some() };

        debug!(target: log::THREAD, method=format!("{}.{}{}", &class.name, &method2.name, method2.descriptor.descriptor()), "Native function invoking JVM method");
        self.stack.push(Frame {
            class: "<native-callback>".to_string(),
            const_pool: 0 as *const ConstPool,
            method: 0 as *const Method,
            operand_stack: OperandStack::new(),
            local_vars: LocalVars::new(),
            pc: 0,
            native: None,
            native_args: vec![],
            native_roots: HashSet::with_hasher(BuildNoHashHasher::default()),
            native_ex: None,
        });

        let depth = self.stack.len();

        if method2.is_synchronized {
            let monitor_ref = if method2.is_static {
                self.runtime.heap.get_static(class)
            } else {
                args[0].reference().clone()
            };
            self.enter_monitor(monitor_ref);
        }

        self.push_frame(class.name.clone(), &class.const_pool as *const ConstPool, method, args);


        while self.stack.len() > depth {
            self.next();

        }

        // We've hit our native stub frame with the result.
        let frame = self.stack.last_mut().unwrap();
        let return_value = if let Some(thrown) = frame.native_ex {
            // We've hit an exception!
            (None, Some(Value::Reference(thrown)))
        } else if has_return {
            let value = frame.operand_stack.pop();
            (Some(value), None)
        } else {
            (None, None)
        };
        self.stack.pop();
        return_value
    }

    pub fn new(name: String, reference: Option<Reference>, runtime: Arc<Runtime>,
               class: String, pool: *const ConstPool, method: *const Method, args: Vec<Value>) -> Arc<Self> {
        reference.map(|reference| {
            runtime.threads.insert(name.clone(), ThreadWait::new(runtime.clone(), reference.clone()))
        });

        let mut frame = Frame {
            class,
            const_pool: pool,
            operand_stack: OperandStack::new(),
            local_vars: LocalVars::new(),
            method,
            pc: 0,
            native: None,
            native_args: vec![],
            native_roots: HashSet::with_hasher(BuildNoHashHasher::default()),
            native_ex: None,
        };
        let mut i = 0;
        for arg in &args {
            frame.local_vars.store_value(i, *arg);
            i += arg.category() as u16;
        }

        let thread = Arc::new(Thread {
            name: name.clone(),
            reference,
            locks: HashMap::with_hasher(BuildNoHashHasher::default()),
            safe: Safe::new(name.clone()),
            runtime: runtime.clone(),
            stack: vec![frame],
        });

        runtime.threads2.write().unwrap().push(thread.clone());

        thread
    }

    pub fn run(&mut self) {
        self.reference.map(|r| self.runtime.heap.start_thread(r));
        let class_name = self.stack.last().unwrap().class.as_str();
        let method = unsafe { self.stack.last().unwrap().method.as_ref().unwrap() };
        let method_name = format!("{}.{}{}", class_name, method.name.as_str(), method.descriptor.descriptor());

        debug!(target: log::THREAD, method=method_name, "Starting thread");
        while !self.stack.is_empty() {
            self.next();
        }
        debug!(target: log::THREAD, method=method_name, "Ended thread");
        // Forever safe!
        self.safe.enter();

        self.reference.map(|r| {
            self.runtime.heap.end_thread(r);
            self.runtime.threads.get(&self.name).unwrap().end();
        });
    }

    pub fn next(&mut self) {

        self.safe.safe_region();

        let curr_frame = self.stack.last_mut().unwrap();
        let method = unsafe { curr_frame.method.as_ref().unwrap() };

        // Handle native methods here.
        if curr_frame.native.is_some() {
            let args = curr_frame.native_args.clone();
            let plugin = curr_frame.native.as_ref().unwrap().clone();
            let thread = self as *const Thread;
            let (result, ex) = (plugin).call(
                method,
                &Args {
                    thread,
                    runtime: self.runtime.clone(),
                    params: args,
                }
            );
            self.stack.pop();
            if let Some(ex) = ex {
                // Invoke throwing.
                let robusta_class = self.runtime.method_area.load_class("com.jkitch.robusta.Robusta");
                let throw_method = robusta_class.find_method(&MethodKey {
                    class: "com.jkitch.robusta.Robusta".to_string(),
                    name: "throwThrowable".to_string(),
                    descriptor: MethodType::from_descriptor("(Ljava/lang/Throwable;)V").unwrap(),
                }).unwrap();
                self.push_frame(robusta_class.name.clone(), &robusta_class.const_pool as *const ConstPool, throw_method as *const Method, vec![ex]);
            } else if let Some(result) = result {
                let frame = self.stack.last_mut().unwrap();
                frame.operand_stack.push(result);
            }
            return;
        }

        // Handle non native methods here.
        instruction(self);
    }

    /// Push a new frame onto the top of the stack.
    pub fn push_frame(&mut self, class: String, const_pool: *const ConstPool, method: *const Method, args: Vec<Value>) {
        let mut frame = Frame {
            class,
            const_pool,
            local_vars: LocalVars::new(),
            operand_stack: OperandStack::new(),
            pc: 0,
            method,
            native: None,
            native_args: vec![],
            native_roots: HashSet::with_hasher(BuildNoHashHasher::default()),
            native_ex: None,
        };

        let mut idx = 0;
        for arg in args {
            frame.local_vars.store_value(idx, arg);
            idx += arg.category() as u16;
        }

        self.stack.push(frame);
    }
    /// Push a new frame onto the top of the stack.
    pub fn push_native(&mut self, class: String, const_pool: *const ConstPool, method: *const Method, args: Vec<Value>, plugin: Arc<dyn Plugin>) {
        let mut frame = Frame {
            class,
            const_pool,
            local_vars: LocalVars::new(),
            operand_stack: OperandStack::new(),
            pc: 0,
            method,
            native: Some(plugin),
            native_args: args.clone(),
            native_roots: HashSet::with_hasher(BuildNoHashHasher::default()),
            native_ex: None,
        };

        let mut idx = 0;
        for arg in args {
            frame.local_vars.store_value(idx, arg.clone());
            if let Value::Reference(reference) = arg {
                frame.native_roots.insert(reference.0);
            }
            idx += arg.category() as u16;
        }

        self.stack.push(frame);
    }

}

/// A single frame in a JVM thread's stack.
pub struct Frame {
    pub class: String,
    /// A reference to the related class's constant pool.
    pub const_pool: *const ConstPool,
    /// A reference to the related method.
    pub method: *const Method,
    pub operand_stack: OperandStack,
    pub local_vars: LocalVars,
    /// The program counter within the current method.
    pub pc: usize,

    /// For native methods only.
    pub native: Option<Arc<dyn Plugin>>,
    pub native_args: Vec<Value>,
    pub native_roots: HashSet<u32, BuildNoHashHasher<u32>>,
    pub native_ex: Option<Reference>,
}

unsafe impl Send for Frame {}

impl Frame {
    fn code(&self) -> &[u8] {
        let method = unsafe { self.method.as_ref().unwrap() };
        &method.code.as_ref().unwrap().code
    }

    pub fn read_i32(&mut self) -> i32 {
        let bytes = &self.code()[self.pc..self.pc + 4];
        let i32 = i32::from_be_bytes(bytes.try_into().unwrap());
        self.pc += 4;
        i32
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = self.code()[self.pc];
        self.pc += 1;
        byte
    }

    pub fn read_i8(&mut self) -> i8 {
        let byte = self.code()[self.pc];
        self.pc += 1;
        i8::from_be_bytes([byte])
    }

    pub fn read_u16(&mut self) -> u16 {
        let bytes = &self.code()[self.pc..self.pc + 2];
        let u16 = u16::from_be_bytes(bytes.try_into().unwrap());
        self.pc += 2;
        u16
    }

    pub fn read_i16(&mut self) -> i16 {
        let bytes = &self.code()[self.pc..self.pc + 2];
        let i16 = i16::from_be_bytes(bytes.try_into().unwrap());
        self.pc += 2;
        i16
    }

    pub fn pop_args(&mut self, is_static: bool, descriptor: &MethodType) -> Vec<Value> {
        let mut values: Vec<Value> = descriptor.parameters.iter().rev().map(|_| {
            self.operand_stack.pop()
        }).collect();
        if !is_static {
            values.push(self.operand_stack.pop());
        }
        values.reverse();
        values
    }
}

/// An operand stack is used to push and pop temporary results in a frame.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.2).
pub struct OperandStack {
    stack: Vec<Value>,
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: vec![] }
    }

    /// Get the roots out of the operand stack.
    pub fn roots(&self) -> HashSet<u32, BuildNoHashHasher<u32>> {
        self.stack.iter()
            .filter_map(|v| {
                match v {
                    Value::Reference(reference) => Some(reference.0),
                    _ => None
                }
            })
            .collect()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

/// Each frame contains an array of variables called the local variables.
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.6.1).
pub struct LocalVars {
    // Very important we track which of these are references!
    map: HashMap<u16, Value, BuildNoHashHasher<u16>>,
}

impl LocalVars {
    pub fn new() -> Self {
        LocalVars { map: HashMap::with_hasher(BuildNoHashHasher::default()) }
    }

    /// Get the roots from this local vars for GC.
    pub fn roots(&self) -> HashSet<u32, BuildNoHashHasher<u32>> {
        self.map.values()
            .filter_map(|v| {
                match v {
                    Value::Reference(reference) => Some(reference.0),
                    _ => None
                }
            }).collect()
    }

    /// Store a value in the local vars.
    pub fn store_value(&mut self, index: u16, value: Value) {
        self.map.insert(index, value);
    }

    pub fn store_cat_one(&mut self, index: u16, value: CategoryOne) {
        self.store_value(index, Value::Int(value.int()));
    }

    /// Load an int from the local vars.
    pub fn load_cat_one(&mut self, index: u16) -> CategoryOne {
        match self.map.get(&index).unwrap() {
            Value::Int(int) => CategoryOne { int: *int },
            Value::Reference(reference) => CategoryOne { reference: *reference },
            _ => panic!("expected to load cat one")
        }
    }

    pub fn load_value(&mut self, index: u16) -> Value {
        self.map.get(&index).unwrap().clone()
    }
}