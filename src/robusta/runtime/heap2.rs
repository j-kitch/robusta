// use std::env::set_var;
// use std::io::Write;
// use std::marker::PhantomData;
// use std::mem::size_of;
// use std::ptr::NonNull;
// use std::sync::{Arc, RwLock};
//
// use crate::collection::AppendMap;
// use crate::java::{Double, FieldType, Float, Int, Long, Reference, Value};
// use crate::runtime::const_pool::Field;
// use crate::runtime::method_area::Class;
//
// const ALIGN: usize = 4;
//
// /// (8 byte primitive) * Integer.MAX_VALUE
// const ARRAY_MAX_LENGTH: u64 = 17_179_869_176;
//
// struct Heap {
//     info: Arc<AppendMap<String, ClassInfo>>,
//     allocator: Allocator,
// }
//
// impl Heap {
//     fn add_class(&self, class: Arc<Class>) -> Arc<ClassInfo> {
//         let (a, b) = self.info.clone().get_or_insert(&class.name, || {
//             let parent = if let Some(parent) = &class.super_class {
//                 Some(self.add_class(parent.clone()))
//             } else {
//                 None
//             };
//
//             // We know the parent width is aligned.
//             let parent_width = parent.as_ref().map(|p| p.width).unwrap_or_else(|| 0);
//
//             let mut our_fields: Vec<FieldInfo> = class.fields.iter()
//                 .map(|f| FieldInfo { name: f.name.clone(), descriptor: f.descriptor.clone(), offset: 0, width: f.descriptor.width() })
//                 .collect();
//
//             // Order fields to achieve good alignment - Largest to smallest.
//             our_fields.sort_by(|f1, f2| f1.width.cmp(&f2.width).reverse());
//
//             // Set the offsets of our fields.
//             let mut offset = parent_width;
//             for field in &mut our_fields {
//                 field.offset = offset;
//                 offset += field.width;
//             }
//
//             // Get our final padded width.
//             let padding = ALIGN - (offset % ALIGN);
//             let width = offset + padding;
//
//             let mut fields = our_fields.into_iter()
//                 .map(Arc::new)
//                 .collect();
//
//             ClassInfo {
//                 name: class.name.clone(),
//                 parent,
//                 fields,
//                 width,
//             }
//         });
//
//         a
//     }
// }
//
// #[derive(Clone)]
// struct ClassInfo {
//     name: String,
//     parent: Option<Arc<ClassInfo>>,
//     /// The fields in this class, does not include parent fields.
//     fields: Vec<Arc<FieldInfo>>,
//     /// The total width of an object data in memory, always aligned to [`ALIGN`].
//     ///
//     /// Note that this **does not** include the width of the object header, only the width
//     /// from the [`Object`]'s `data` field.
//     width: usize,
// }
//
// #[derive(Clone)]
// struct FieldInfo {
//     name: String,
//     /// The type of the field.
//     descriptor: FieldType,
//     /// The offset of the field in the object.
//     offset: usize,
//     /// The width (in bytes) of this field in the object.
//     width: usize,
// }
//
// #[repr(C)]
// #[derive(Clone)]
// struct Array {
//     /// The type of the components of this array.
//     component: FieldType,
//     /// The length (in bytes) of this array, the stride is determined from the
//     /// component type.
//     length: u64,
//     /// A pointer to the array.
//     data: *mut u8,
// }
//
// fn write_value(ptr: *mut u8, offset: usize, value: Value, descriptor: &FieldType) {
//     unsafe {
//         let ptr = ptr.add(offset);
//         match descriptor {
//             FieldType::Boolean | FieldType::Byte => {
//                 let byte = value.int().0 as i8;
//                 let ptr: *mut i8 = ptr.cast();
//                 ptr.write(byte)
//             }
//             FieldType::Char => {
//                 let char = value.int().0 as u16;
//                 let ptr: *mut u16 = ptr.cast();
//                 ptr.write(char)
//             }
//             FieldType::Short => {
//                 let short = value.int().0 as i16;
//                 let ptr: *mut i16 = ptr.cast();
//                 ptr.write(short)
//             }
//             FieldType::Int => {
//                 let int = value.int().0;
//                 let ptr: *mut i32 = ptr.cast();
//                 ptr.write(int)
//             }
//             FieldType::Long => {
//                 let long = value.long().0;
//                 let ptr: *mut i64 = ptr.cast();
//                 ptr.write(long)
//             }
//             FieldType::Float => {
//                 let float = value.float().0;
//                 let ptr: *mut f32 = ptr.cast();
//                 ptr.write(float)
//             }
//             FieldType::Double => {
//                 let double = value.double().0;
//                 let ptr: *mut f64 = ptr.cast();
//                 ptr.write(double)
//             }
//             FieldType::Reference(_) | FieldType::Array(_) => {
//                 let reference = value.reference().0;
//                 let ptr: *mut u32 = ptr.cast();
//                 ptr.write(reference)
//             }
//         }
//     }
// }
//
// fn get_value(ptr: *mut u8, offset: usize, descriptor: &FieldType) -> Value {
//     unsafe {
//         let ptr = ptr.add(offset);
//         match descriptor {
//             FieldType::Boolean | FieldType::Byte => {
//                 let byte: i8 = *ptr.cast();
//                 Value::Int(Int(byte as i32))
//             }
//             FieldType::Char => {
//                 let char: u16 = *ptr.cast();
//                 Value::Int(Int(char as i32))
//             }
//             FieldType::Short => {
//                 let short: i16 = *ptr.cast();
//                 Value::Int(Int(short as i32))
//             }
//             FieldType::Int => {
//                 let int: i32 = *ptr.cast();
//                 Value::Int(Int(int))
//             }
//             FieldType::Long => {
//                 let long: i64 = *ptr.cast();
//                 Value::Long(Long(long))
//             }
//             FieldType::Float => {
//                 let float: f32 = *ptr.cast();
//                 Value::Float(Float(float))
//             }
//             FieldType::Double => {
//                 let double: f64 = *ptr.cast();
//                 Value::Double(Double(double))
//             }
//             FieldType::Reference(_) | FieldType::Array(_) => {
//                 let reference: u32 = *ptr.cast();
//                 Value::Reference(Reference(reference))
//             }
//         }
//     }
// }
//
// impl Array {
//     fn length(&self) -> Int {
//         let width = self.component.width() as u64;
//         let length = self.length / width;
//         Int(length as i32)
//     }
//
//     fn get_element(&self, index: Int, component: FieldType) -> Value {
//         if self.component.arr_comp().ne(&component.arr_comp()) {
//             panic!("Not matching array component types")
//         }
//         let index = index.0 as usize * self.component.width();
//         get_value(self.data, index, &self.component)
//     }
//
//     fn set_element(&self, index: Int, component: &FieldType, value: Value) {
//         if self.component.arr_comp().ne(&component.arr_comp()) {
//             panic!("Not matching array component types")
//         }
//         let index = index.0 as usize * self.component.width();
//         write_value(self.data, index, value, &self.component)
//     }
// }
//
// #[repr(C)]
// #[derive(Clone)]
// struct Object {
//     class: Arc<ClassInfo>,
//     data: *mut u8,
// }
//
// impl Object {
//     fn find_field(&self, field: &Field) -> &FieldInfo {
//         let class_name = field.class.name.as_str();
//
//         let mut class = Some(&self.class);
//         while let Some(curr_class) = class {
//             if curr_class.name.eq(class_name) {
//                 break;
//             }
//             class = curr_class.parent.as_ref();
//         }
//
//         for f in &class.unwrap().fields {
//             if f.name.eq(field.name.as_str()) && f.descriptor.eq(&field.descriptor) {
//                 return f.as_ref();
//             }
//         }
//
//         panic!("Could not find field {:?} in heap object", field)
//     }
//
//     fn get_field(&self, field: &Field) -> Value {
//         let field = self.find_field(field);
//         get_value(self.data, field.offset, &field.descriptor)
//     }
//
//     fn set_field(&self, field: &Field, value: Value) {
//         let field = self.find_field(field);
//         write_value(self.data, field.offset, value, &field.descriptor)
//     }
// }
//
// struct Allocator {
//     data: RwLock<Vec<u8>>,
// }
//
// impl Allocator {
//     fn allocate_object(&self, class: &Arc<ClassInfo>) -> Object {
//         // Need to allocate space for the header and the data.
//         let header_width = size_of::<Object>() - size_of::<*mut u8>();
//
//         let offset = {
//             let mut data = self.data.write().unwrap();
//             let offset = data.len();
//             data.append(&mut vec![0; class.width]);
//             offset
//         };
//
//         let data = self.data.read().unwrap();
//         let ptr = data.as_ptr();
//         unsafe {
//             let ptr = ptr.add(offset);
//             let ptr = ptr.cast_mut();
//             let object_ptr: *mut Object = ptr.cast();
//             let data_ptr: *mut u8 = object_ptr.add(size_of::<Arc<Class>>()).cast();
//
//             let obj = Object { class: class.clone(), data: data_ptr };
//             *(object_ptr) = obj.clone();
//             obj.clone()
//         }
//     }
//
//     fn allocate_array(&self, component: &FieldType, length: Int) -> Array {
//
//     }
// }
//
