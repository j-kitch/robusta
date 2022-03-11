use crate::descriptor::MethodDescriptor;
use crate::heap::Value;
use crate::heap::Value::Ref;
use crate::native::NativePlugin;
use crate::runtime::Runtime;

pub struct ClassPlugin {}

impl ClassPlugin {
    pub fn new() -> Self {
        ClassPlugin {}
    }
}

impl NativePlugin for ClassPlugin {
    fn supports(&self, class: &str, name: &str, desc: &MethodDescriptor) -> bool {
        class.eq("java/lang/Object") &&
            name.eq("getClass") &&
            desc.descriptor().eq("()Ljava/lang/Class;")
    }

    fn invoke(&mut self, runtime: &mut Runtime, args: Vec<Value>) -> Option<Value> {
        let obj_ref = args[0].reference();
        let obj = runtime.heap.get(obj_ref);
        let obj = obj.as_ref().borrow();
        let obj = obj.obj();

        let this_class = obj.class.clone();

        let class_obj_ref = runtime.create_class_object(this_class);

        Some(Ref(class_obj_ref))
    }
}
