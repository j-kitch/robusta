//! Value types are the computational types that are used at runtime, in a JVM.
//!
//! The mapping between compile time and runtime types is defined in the [mapping table](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-2.html#jvms-2.11.1-320).

#[derive(Clone, Copy, Debug, PartialEq)]
/// Int is the 32-bit signed integer in the JVM.
pub struct Int(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Long is the 64-bit signed integer in the JVM.
pub struct Long(pub i64);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Float is the 32-bit floating-point number in the JVM.
pub struct Float(pub f32);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Double is the 64-bit floating-point number in the JVM.
pub struct Double(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Reference is the reference type in the JVM.  References are 32-bit unsigned integers
/// that key into the heap.
///
/// We are using 32 bit integers to match the "category-1" size in the specification, this
/// might be something we decide to change later.
pub struct Reference(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Return Address is the address of an opcode instruction in a method.
///
/// We are using 32 bit unsigned integers to match the "category-1" size in the specification,
/// this might be something we decide to change later.
pub struct ReturnAddress(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
/// A generic enumeration over the potential runtime values in the JVM.
pub enum Value {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
    ReturnAddress(ReturnAddress),
}

impl Value {
    pub fn int(&self) -> Int {
        match self {
            Value::Int(int) => int.clone(),
            _ => panic!("expected int")
        }
    }

    pub fn long(&self) -> Long {
        match self {
            Value::Long(long) => long.clone(),
            _ => panic!("expected int")
        }
    }

    pub fn float(&self) -> Float {
        match self {
            Value::Float(float) => float.clone(),
            _ => panic!("expected int")
        }
    }

    pub fn double(&self) -> Double {
        match self {
            Value::Double(double) => double.clone(),
            _ => panic!("expected int")
        }
    }

    pub fn reference(&self) -> Reference {
        match self {
            Value::Reference(reference) => reference.clone(),
            _ => panic!("expected int")
        }
    }

    pub fn category(&self) -> usize {
        match self {
            Value::Long(_) | Value::Double(_) => 2,
            _ => 1,
        }
    }

    pub fn cat_one(&self) -> CategoryOne {
        match self {
            Value::Int(int) => CategoryOne { int: *int },
            Value::Float(float) => CategoryOne { float: *float },
            Value::Reference(reference) => CategoryOne { reference: *reference },
            _ => panic!("Not a category one type {:?}", self)
        }
    }
}

#[derive(Clone, Copy)]
/// A union of category one types in the JVM.
pub union CategoryOne {
    pub int: Int,
    pub float: Float,
    pub reference: Reference,
    pub return_address: ReturnAddress,
}

impl CategoryOne {
    pub fn int(&self) -> Int {
        unsafe { self.int }
    }

    pub fn reference(&self) -> Reference {
        unsafe { self.reference }
    }
}

/// A union of category two types in the JVM.
pub union CategoryTwo {
    pub long: Long,
    pub double: Double,
}

impl CategoryTwo {
    pub fn long(&self) -> Long {
        unsafe { self.long }
    }
}