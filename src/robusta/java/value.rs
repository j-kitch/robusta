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

/// A union of category one types in the JVM.
pub union CategoryOne {
    pub int: Int,
    pub float: Float,
    pub reference: Reference,
    pub return_address: ReturnAddress,
}

/// A union of category two types in the JVM.
pub union CategoryTwo {
    pub long: Long,
    pub double: Double,
}