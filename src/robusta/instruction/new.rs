use std::sync::Arc;
use std::thread::spawn;

use crate::java::{CategoryOne, Value};
use crate::runtime::{heap, Method, Runtime};
use crate::runtime::heap3::ArrayType;
use crate::runtime::method_area::Field;
use crate::thread::Thread;

// /// Many instructions involve resolving class types, which follows a strict algorithm to ensure that
// /// classes are loaded in the correct order.
// ///
// /// This function handles this process.
// pub fn resolve_class(runtime: Arc<Runtime>, class: &str) {
//     let fin_resolved = runtime.method_area.try_resolve(class);
//     if fin_resolved.is_none() {
//         return;
//     }
//
//     // Load class, and all it's superclasses into the method area.
//     let (class, _) = runtime.method_area.insert(runtime.clone(), class);
//
//     // Classes that this thread needs to initialize.
//     let to_init = runtime.method_area.try_start_init(class.name.as_str());
//
//     let classes = to_init.iter().map(|t| t.0.clone()).collect();
//     let mut clinit_thread = Thread::clinit(runtime.clone(), classes);
//     spawn(move || clinit_thread.run()).join().unwrap();
//
//     for (_, sender) in to_init {
//         sender.send(()).unwrap();
//     }
//
//     // Also want to resolve all the field & method types in the class
//     for field in class.fields.iter() {
//         resolve_field(runtime.clone(), field);
//     }
//
//     // And resolve all the methods!
//     for method in class.methods.iter() {
//         resolve_method(runtime.clone(), method);
//     }
//
//     fin_resolved.unwrap().send(()).unwrap();
// }

/// Like resolving a class, but we need to resolve every type that is in the method signature!
// pub fn resolve_method(runtime: Arc<Runtime>, method: &Arc<Method>) {
//     for class in method.descriptor.class_names() {
//         resolve_class(runtime.clone(), class.as_str())
//     }
// }
//
// pub fn resolve_field(runtime: Arc<Runtime>, field: &Arc<Field>) {
//     if let Some(class) = field.descriptor.class_name() {
//         resolve_class(runtime.clone(), class.as_str())
//     }
// }

/// Instruction `new` creates a new object in the heap.
///
/// Opcode 0xBB
///
/// For further information, see [the spec](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5.new).
pub fn new(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();

    let class_idx = cur_frame.read_u16();
    let class = thread.runtime.method_area.resolve_class(cur_frame.const_pool, class_idx);
    let class = unsafe { class.as_ref().unwrap() };
    // println!("{} new - {}", thread.group.as_str(), class_const.name.as_str());
    //
    // resolve_class(thread.runtime.clone(), class_const.name.as_str());
    // let (class, _) = thread.runtime.method_area.insert(thread.runtime.clone(), class_const.name.as_str());
    let new_ref = thread.runtime.heap.new_object(class);

    cur_frame.operand_stack.push_cat_one(CategoryOne { reference: new_ref });
}

pub fn new_array(thread: &mut Thread) {
    let cur_frame = thread.stack.last_mut().unwrap();
    let array_type = cur_frame.read_u8();

    let count = cur_frame.operand_stack.pop_cat_one().int();

    let arr_ref = match array_type {
        5 => thread.runtime.heap.new_array(crate::heap::allocator::ArrayType::Char, count),
        _ => panic!("newarray has not been implemented for array type {}", array_type)
    };

    cur_frame.operand_stack.push_cat_one(CategoryOne { reference: arr_ref });
}