use std::mem::size_of;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::collection::AppendMap;
use crate::java::{Double, FieldType, Float, Int, Long, Reference, Value};
use crate::runtime::const_pool::Field;
use crate::runtime::method_area::Class;

const ALIGN: usize = 4;

#[allow(dead_code)]
/// (8 byte primitive) * Integer.MAX_VALUE
const ARRAY_MAX_LENGTH: u64 = 17_179_869_176;

pub struct HeapInner {
    info: Arc<AppendMap<String, ClassInfo>>,
    pub allocator: Allocator,
}

impl HeapInner {
    pub fn new() -> Self {
        HeapInner {
            info: AppendMap::new(),
            allocator: Allocator {
                data: vec![0; HEAP_SIZE].into_boxed_slice(),
                used: AtomicUsize::new(0),
            },
        }
    }

    pub fn print_stats(&self) {
        let used = self.allocator.used.load(Ordering::SeqCst);
        let fraction = 100.0 * (used as f64) / HEAP_SIZE as f64;
        println!("Used {} bytes of heap ({:.2}%)", used, fraction)
    }

    pub fn add_class(&self, class: Arc<Class>) -> Arc<ClassInfo> {
        let (a, _) = self.info.clone().get_or_insert(&class.name, || {
            let parent = if let Some(parent) = &class.super_class {
                Some(self.add_class(parent.clone()))
            } else {
                None
            };

            // We know the parent width is aligned.
            let parent_width = parent.as_ref().map(|p| p.width).unwrap_or_else(|| 0);

            let mut our_fields: Vec<FieldInfo> = class.fields.iter()
                .map(|f| FieldInfo { name: f.name.clone(), descriptor: f.descriptor.clone(), offset: 0, width: f.descriptor.width() })
                .collect();

            // Order fields to achieve good alignment - Largest to smallest.
            our_fields.sort_by(|f1, f2| f1.width.cmp(&f2.width).reverse());

            // Set the offsets of our fields.
            let mut offset = parent_width;
            for field in &mut our_fields {
                field.offset = offset;
                offset += field.width;
            }

            // Get our final padded width.
            let padding = ALIGN - (offset % ALIGN);
            let width = offset + padding;

            let fields = our_fields.into_iter()
                .map(Arc::new)
                .collect();

            ClassInfo {
                name: class.name.clone(),
                parent,
                fields,
                width,
            }
        });

        a
    }
}

#[derive(Clone)]
pub struct ClassInfo {
    pub name: String,
    parent: Option<Arc<ClassInfo>>,
    /// The fields in this class, does not include parent fields.
    fields: Vec<Arc<FieldInfo>>,
    /// The total width of an object data in memory, always aligned to [`ALIGN`].
    ///
    /// Note that this **does not** include the width of the object header, only the width
    /// of the [`ObjectData`].
    width: usize,
}

impl ClassInfo {
    /// Find parents starting from the given class.
    pub fn find_parents_from(self: &Arc<Self>, name: &str) -> Vec<Arc<ClassInfo>> {
        let mut parents = Vec::new();
        let mut found = false;
        let mut class = Some(self);
        while let Some(current) = class {
            if !found {
                found = current.name.eq(name);
            }
            if found {
                parents.push(current.clone());
            }
            class = current.parent.as_ref();
        }
        parents
    }

    pub fn find_parent(self: &Arc<Self>, name: &str) -> Option<Arc<ClassInfo>> {
        let mut class = Some(self);
        while let Some(current) = class {
            if current.name.eq(name) {
                return Some(current.clone());
            }
            class = current.parent.as_ref();
        }
        None
    }
}

#[derive(Clone)]
struct FieldInfo {
    name: String,
    /// The type of the field.
    descriptor: FieldType,
    /// The offset of the field in the object.
    offset: usize,
    /// The width (in bytes) of this field in the object.
    width: usize,
}

#[repr(C)]
#[derive(Clone)]
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
    pub fn class(&self) -> Arc<ClassInfo> {
        let header = self.header();
        header.class.clone()
    }

    fn header(&self) -> &ObjectHeader {
        unsafe { self.header.as_ref().unwrap() }
    }

    fn find_field(&self, field: &Field) -> Arc<FieldInfo> {
        let header = self.header();

        let classes = header.class.find_parents_from(field.class.name.as_str());

        classes.iter()
            .flat_map(|c| c.fields.iter())
            .find(|f| f.name.eq(field.name.as_str()) && f.descriptor.eq(&field.descriptor))
            .expect("Could not find field")
            .clone()
    }

    pub fn get_field(&self, field: &Field) -> Value {
        let field = self.find_field(field);

        read_value(self.data, field.offset, &field.descriptor)
    }

    pub fn set_field(&self, field: &Field, value: Value) {
        let field = self.find_field(field);

        write_value(self.data, field.offset, &field.descriptor, value)
    }
}

#[repr(C)]
#[derive(Clone)]
/// The object header is used to index into an object.
///
/// TODO: This will be used for GC information.
struct ObjectHeader {
    /// The class of this object
    class: Arc<ClassInfo>,
}

#[repr(C)]
#[derive(Clone)]
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

    pub fn set_element(&self, index: Int, value: Value) {
        let header = self.header();
        let index = index.0 as usize * header.component.width();
        write_value(self.data, index, &header.component.to_field(), value)
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
}

/// The default heap size of the openjdk is 1280MB
const HEAP_SIZE: usize = 1280 * 1024 * 1024;

/// The allocator is the actual heap memory that is used for storing objects.
pub struct Allocator {
    data: Box<[u8]>,
    used: AtomicUsize,
}

impl Allocator {
    pub fn new_object(&self, class: Arc<ClassInfo>) -> Object {
        let header_size = size_of::<ObjectHeader>();
        let data_size = class.width;
        let size = header_size + data_size;

        let start_ptr = self.allocate(size);

        unsafe {
            let object = Object {
                header: start_ptr.cast(),
                data: start_ptr.add(header_size),
            };

            object.header.write(ObjectHeader { class });
            object.data.write_bytes(0, object.header.read().class.width);

            object
        }
    }

    pub fn new_array(&self, component: ArrayType, length: Int) -> Array {
        let header_size = size_of::<ArrayHeader>();
        let data_size = length.0 as usize * component.width();
        let size = header_size + data_size;

        let start_ptr = self.allocate(size);

        unsafe {
            let array = Array {
                header: start_ptr.cast(),
                data: start_ptr.add(header_size),
            };

            array.header.write(ArrayHeader { component, length: data_size });
            array.data.write_bytes(0, size);

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

        unsafe {
            self.data.as_ptr().add(start_of_memory).cast_mut()
        }
    }
}

fn write_value(data_start: *mut u8, offset: usize, field: &FieldType, value: Value) {
    unsafe {
        let pointer: *mut u8 = data_start.add(offset);
        match field {
            FieldType::Boolean | FieldType::Byte => {
                let pointer: *mut i8 = pointer.cast();
                pointer.write(value.int().0 as i8)
            }
            FieldType::Char => {
                let pointer: *mut u16 = pointer.cast();
                pointer.write(value.int().0 as u16)
            }
            FieldType::Short => {
                let pointer: *mut i16 = pointer.cast();
                pointer.write(value.int().0 as i16)
            }
            FieldType::Int => {
                let pointer: *mut i32 = pointer.cast();
                pointer.write(value.int().0)
            }
            FieldType::Long => {
                let pointer: *mut i64 = pointer.cast();
                pointer.write(value.long().0 as i64)
            }
            FieldType::Float => {
                let pointer: *mut f32 = pointer.cast();
                pointer.write(value.float().0 as f32)
            }
            FieldType::Double => {
                let pointer: *mut f64 = pointer.cast();
                pointer.write(value.double().0 as f64)
            }
            FieldType::Reference(_) | FieldType::Array(_) => {
                let pointer: *mut u32 = pointer.cast();
                pointer.write(value.reference().0 as u32)
            }
        }
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
