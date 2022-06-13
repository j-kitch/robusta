use std::collections::HashMap;
use std::rc::Rc;
use crate::class::{Class, Const, Method, MethodRef};
use crate::descriptor::MethodDescriptor;
use crate::robusta::class_file::Version;
use crate::thread::Frame;
use crate::thread::local_vars::LocalVars;
use crate::thread::op_stack::OperandStack;

pub fn init_parents_frame(parents: &[String]) -> Frame {
    let mut method = Method {
        name: "<invoke_clinit>".to_string(),
        descriptor: MethodDescriptor::parse("()V"),
        native: false,
        max_stack: 0,
        max_locals: 0,
        code: vec![]
    };

    for (idx, _) in parents.iter().enumerate() {
        method.code.push(0xCA);

        let idx = idx as u16 + 1;
        let idx_bytes = idx.to_be_bytes();
        method.code.push(idx_bytes[0]);
        method.code.push(idx_bytes[1]);

        method.code.push(0xB8);
        method.code.push(idx_bytes[0]);
        method.code.push(idx_bytes[1]);
    }

    method.code.push(0xB1);

    let method = Rc::new(method);

    let mut class = Class {
        version: Version { minor: 0, major: 0 },
        const_pool: HashMap::new(),
        access_flags: 0,
        this_class: "<shim>".to_string(),
        super_class: None,
        interfaces: vec![],
        fields: vec![],
        methods: vec![method.clone()],
    };

    for (idx, parent) in parents.iter().enumerate() {
        let idx = idx as u16 + 1;
        class.const_pool.insert(idx, Const::Method(MethodRef {
            class: parent.clone(),
            name: "<clinit>".to_string(),
            descriptor: MethodDescriptor::parse("()V"),
        }));
    }

    Frame {
        pc: 0,
        class: Rc::new(class),
        method: method.clone(),
        local_vars: LocalVars::new(0),
        op_stack: OperandStack::new(0),
    }
}
