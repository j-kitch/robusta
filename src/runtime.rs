use std::cell::RefCell;
use std::rc::Rc;
use crate::class::Class;
use crate::class_loader::ClassLoader;
use crate::heap::{Heap, Ref, Value};
use crate::native::NativeMethods;

pub struct Runtime {
    pub class_loader: ClassLoader,
    pub heap: Heap,
    pub native: NativeMethods,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            class_loader: ClassLoader::new(),
            heap: Heap::new(),
            native: NativeMethods::load(),
        }
    }

    pub fn load_class(&mut self, class_name: &str) -> Rc<Class> {
        self.class_loader.load(class_name).unwrap()
    }

    pub fn create_object(&mut self, class: Rc<Class>) -> (u32, Rc<RefCell<Ref>>) {
        self.heap.create(class)
    }

    pub fn insert_char_array(&mut self, char_arr: Vec<u16>) -> u32 {
        self.heap.insert_char_array(char_arr)
    }

    pub fn insert_ref_array(&mut self, ref_arr: Vec<u32>) -> u32 {
        self.heap.insert_ref_array(ref_arr)
    }

    pub fn insert_str_const(&mut self, string: &str) -> u32 {
        let string_class = self.load_class("java/lang/String");
        let (str_ref, str_obj) = self.create_object(string_class.clone());

        let chars: Vec<u16> = string.encode_utf16().collect();
        let chars_ref = self.insert_char_array(chars);

        let mut str_obj = str_obj.borrow_mut();

        let str_obj = match &mut *str_obj {
            Ref::Obj(obj) => obj,
            _ => panic!("")
        };

        let mut chars_field = str_obj.fields.iter_mut()
            .find(|f| f.field.name.eq("chars"))
            .unwrap();

        chars_field.value = Value::Ref(chars_ref);

        str_ref
    }
}
