use std::mem::size_of;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::sync::atomic::{AtomicUsize, Ordering};

use tracing::{debug, trace};

use crate::heap::hash_code::HashCode;
use crate::java::{CategoryOne, Double, FieldType, Float, Int, Long, Reference, Value};
use crate::log;
use crate::method_area::Class;
use crate::method_area::const_pool::FieldKey;

#[repr(C)]
#[derive(Clone, Copy)]
/// An object is a reference to an object in the heap.
///
/// This value must be sized, copyable and shareable within the heap data structures,
/// but the actual contents of this type lie solely within the ownership of the heap!
///
/// The actual layout in the heap will be `[header bytes ... ] [data bytes ... ]`
pub struct Object {
    /// The object header.
    header: *mut ObjectHeader,
    /// The internal data for fields.
    ///
    /// The layout of data in this field is as follows:
    /// In hierarchy order of classes: (java.lang.Object, ..., thisClass):
    ///    Fields are minimized to their compile time size
    ///        (byte -> 1 byte, short -> 2 bytes, etc...)
    ///   Sort the classes declared fields in reverse width order
    ///        (longs & doubles, then ints & floats, then chars & shorts, ...)
    ///   Lay each field out in order.
    ///        (Attempting to minimize fragmentaton of data within the object).
    ///   Add alignment padding to the end of the data (if required).
    ///        (For ex, if we end on `[char] [boolean] [xxx 1 byte left xxx]`
    ///             then pad 1 byte.)
    ///   Insert the next classes fields.
    ///
    /// This order should allow superclasses to index into the object without having to
    /// be aware of the child classes layout, and keep fairly good fragmentation avoidance.
    data: *mut u8,
}

unsafe impl Send for Object {}

unsafe impl Sync for Object {}

impl Object {
    pub fn class(&self) -> &Class {
        let header = self.header();
        unsafe { header.class.as_ref().unwrap() }
    }

    fn header(&self) -> &ObjectHeader {
        unsafe { self.header.as_ref().unwrap() }
    }

    pub fn get_field(&self, field: &FieldKey) -> Value {
        let field = self.class().find_field(field);

        read_value(self.data, field.offset, &field.descriptor)
    }

    pub fn set_field(&self, field: &FieldKey, value: CategoryOne) {
        let field = self.class().find_field(field);

        write_value(self.data, field.offset, value)
    }

    pub fn get_static(&self, field: &FieldKey) -> Value {
        let field = self.class().find_static(field);

        read_value(self.data, field.offset, &field.descriptor)
    }

    pub fn set_static(&self, field: &FieldKey, value: CategoryOne) {
        let field = self.class().find_static(field);

        write_value(self.data, field.offset, value)
    }

    pub fn hash_code(&self) -> Int {
        unsafe { (*self.header).hash_code }
    }
}

#[repr(C)]
#[derive(Clone)]
/// The object header is used to index into an object.
///
/// TODO: This will be used for GC information.
struct ObjectHeader {
    /// The class of this object
    class: *const Class,
    hash_code: Int,
}

#[repr(C)]
#[derive(Clone, Copy)]
/// An array object in the heap.
pub struct Array {
    header: *mut ArrayHeader,
    data: *mut u8,
}

unsafe impl Send for Array {}

unsafe impl Sync for Array {}

impl Array {
    fn header(&self) -> &ArrayHeader {
        unsafe {
            self.header.as_ref().unwrap()
        }
    }

    pub fn length(&self) -> Int {
        let header = self.header();
        let length = header.length / header.component.width();
        Int(length as i32)
    }

    pub fn set_element(&self, index: Int, value: CategoryOne) {
        let header = self.header();
        let index = index.0 as usize * header.component.width();
        write_value(self.data, index, value)
    }

    pub fn get_element(&self, index: Int) -> Value {
        let offset = index.0 as usize * self.header().component.width();
        read_value(self.data, offset, &self.header().component.to_field())
    }

    pub fn as_chars_slice(&self) -> &[u16] {
        let header = self.header();
        if header.component.ne(&ArrayType::Char) {
            panic!("cannot export as chars slice")
        }
        let length = header.length / header.component.width();
        let pointer: *mut u16 = self.data.cast();
        unsafe {
            from_raw_parts(pointer.cast_const(), length)
        }
    }

    pub fn as_chars_mut(&self) -> &mut [u16] {
        let header = self.header();
        if header.component.ne(&ArrayType::Char) {
            panic!("cannot export as chars slice")
        }
        let length = header.length / header.component.width();
        let pointer: *mut u16 = self.data.cast();
        unsafe {
            from_raw_parts_mut(pointer, length)
        }
    }
}

#[repr(C)]
#[derive(Clone)]
/// The array header is used to index into the array.
///
/// TODO: This will be used for GC information.
struct ArrayHeader {
    /// The type of the values in the array.
    component: ArrayType,
    /// The length (in bytes) of the array data.
    length: usize,
    hash_code: Int,
}

unsafe impl Send for ArrayHeader {}

unsafe impl Sync for ArrayHeader {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArrayType {
    BooleanOrByte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Reference,
}

impl ArrayType {
    pub fn from(field: &FieldType) -> ArrayType {
        match field {
            FieldType::Boolean | FieldType::Byte => ArrayType::BooleanOrByte,
            FieldType::Char => ArrayType::Char,
            FieldType::Short => ArrayType::Short,
            FieldType::Int => ArrayType::Int,
            FieldType::Long => ArrayType::Long,
            FieldType::Float => ArrayType::Float,
            FieldType::Double => ArrayType::Double,
            FieldType::Array(_) | FieldType::Reference(_) => ArrayType::Reference,
        }
    }

    pub fn to_field(&self) -> FieldType {
        match self {
            ArrayType::BooleanOrByte => FieldType::Byte,
            ArrayType::Char => FieldType::Char,
            ArrayType::Short => FieldType::Short,
            ArrayType::Int => FieldType::Int,
            ArrayType::Long => FieldType::Long,
            ArrayType::Float => FieldType::Float,
            ArrayType::Double => FieldType::Double,
            ArrayType::Reference => FieldType::Reference("".to_string())
        }
    }

    pub fn width(&self) -> usize {
        match self {
            ArrayType::BooleanOrByte => 1,
            ArrayType::Char | ArrayType::Short => 2,
            ArrayType::Long | ArrayType::Double => 8,
            _ => 4,
        }
    }

    pub fn descriptor(&self) -> String {
        match self {
            ArrayType::BooleanOrByte => "[B".to_string(),
            ArrayType::Char => "[C".to_string(),
            ArrayType::Short => "[S".to_string(),
            ArrayType::Int => "[I".to_string(),
            ArrayType::Long => "[L".to_string(),
            ArrayType::Float => "[F".to_string(),
            ArrayType::Double => "[D".to_string(),
            ArrayType::Reference => "[L".to_string(),
        }
    }
}

/// The default heap size of the openjdk is 1280MB
const HEAP_SIZE: usize = 1280 * 1024 * 1024;

/// The allocator is the actual heap memory that is used for storing objects.
pub struct Allocator {
    data: Box<[u8]>,
    used: AtomicUsize,
    hash_code: HashCode,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {
            data: vec![0; HEAP_SIZE].into_boxed_slice(),
            used: AtomicUsize::new(0),
            hash_code: HashCode::new(),
        }
    }

    pub fn print_stats(&self) {
        let used = self.used.load(Ordering::SeqCst);
        let used_mbs = (used / 1024)  / 1024;
        let max = HEAP_SIZE;
        let percentage = 100.0 * (used as f64) / (max as f64);
        debug!(target: log::HEAP, used=format!("{}mb {:.2}%", used_mbs, percentage));
    }

    pub fn new_object(&self, class: &Class) -> Object {
        trace!(target: log::HEAP, class=class.name.as_str(), "Allocating object");
        let header_size = size_of::<ObjectHeader>();
        let data_size = class.instance_width;
        let size = header_size + data_size;

        let start_ptr = self.allocate(size);

        let class_ptr = class as *const Class;

        unsafe {
            let object = Object {
                header: start_ptr.cast(),
                data: start_ptr.add(header_size),
            };

            object.header.write(ObjectHeader { class: class_ptr, hash_code: self.hash_code.next() });
            object.data.write_bytes(0, class.instance_width);

            self.print_stats();

            object
        }
    }

    pub fn new_static_object(&self, class: &Class) -> Object {
        trace!(target: log::HEAP, class=class.name.as_str(), "Allocating static object");
        let header_size = size_of::<ObjectHeader>();
        let data_size = class.static_width;
        let size = header_size + data_size;

        let start_ptr = self.allocate(size);

        let class_ptr = class as *const Class;

        unsafe {
            let object = Object {
                header: start_ptr.cast(),
                data: start_ptr.add(header_size),
            };

            object.header.write(ObjectHeader { class: class_ptr, hash_code: Int(0) });
            object.data.write_bytes(0, class.static_width);

            self.print_stats();

            object
        }
    }

    pub fn new_array(&self, component: ArrayType, length: Int) -> Array {
        trace!(target: log::HEAP, component=component.descriptor(), length=length.0, "Allocating array");
        let header_size = size_of::<ArrayHeader>();
        let data_size = length.0 as usize * component.width();
        let size = header_size + data_size;

        let start_ptr = self.allocate(size);

        unsafe {
            let array = Array {
                header: start_ptr.cast(),
                data: start_ptr.add(header_size),
            };

            array.header.write(ArrayHeader { component, length: data_size, hash_code: self.hash_code.next() });
            array.data.write_bytes(0, size);

            self.print_stats();

            array
        }
    }

    /// Allocate the given number of bytes, returning a pointer to the start.
    fn allocate(&self, size: usize) -> *mut u8 {
        let result = self.used.fetch_update(
            Ordering::SeqCst, Ordering::SeqCst,
            |old_used| old_used.checked_add(size));

        // TODO: Need to add GC.
        let start_of_memory = result.expect("Heap is too full");

        if start_of_memory + size > HEAP_SIZE {
            panic!("Out Of Memory");
        }

        unsafe {
            self.data.as_ptr().add(start_of_memory).cast_mut()
        }
    }
}

fn write_value(data_start: *mut u8, offset: usize, value: CategoryOne) {
    unsafe {
        let pointer: *mut u8 = data_start.add(offset);
        let pointer: *mut i32 = pointer.cast();
        pointer.write(value.int().0)
    }
}

fn read_value(data_start: *mut u8, offset: usize, field: &FieldType) -> Value {
    unsafe {
        let pointer = data_start.add(offset);
        match field {
            FieldType::Boolean => {
                let pointer: *mut i8 = pointer.cast();
                Value::Int(Int(pointer.read() as i32))
            }
            FieldType::Byte => {
                let pointer: *mut i8 = pointer.cast();
                Value::Int(Int(pointer.read() as i32))
            }
            FieldType::Char => {
                let pointer: *mut u16 = pointer.cast();
                Value::Int(Int(pointer.read() as i32))
            }
            FieldType::Short => {
                let pointer: *mut i16 = pointer.cast();
                Value::Int(Int(pointer.read() as i32))
            }
            FieldType::Int => {
                let pointer: *mut i32 = pointer.cast();
                Value::Int(Int(pointer.read()))
            }
            FieldType::Long => {
                let pointer: *mut i64 = pointer.cast();
                Value::Long(Long(pointer.read() as i64))
            }
            FieldType::Float => {
                let pointer: *mut f32 = pointer.cast();
                Value::Float(Float(pointer.read() as f32))
            }
            FieldType::Double => {
                let pointer: *mut f64 = pointer.cast();
                Value::Double(Double(pointer.read()))
            }
            FieldType::Reference(_) | FieldType::Array(_) => {
                let pointer: *mut u32 = pointer.cast();
                Value::Reference(Reference(pointer.read()))
            }
        }
    }
}
