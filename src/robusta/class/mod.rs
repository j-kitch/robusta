use std::fmt::format;
use std::rc::Rc;

use crate::descriptor::Descriptor;

pub mod object;

pub enum Primitive {
    Boolean,
    Byte,
    Short,
    Char,
    Int,
    Long,
    Float,
    Double,
}

pub enum Class {
    Primitive(Primitive),
    Array { component: Box<Class> },
    Object { file: Rc<object::Class> },
}

impl Class {
    pub fn name(&self) -> String {
        match self {
            Class::Object { file } => file.this_class.clone(),
            _ => panic!("error")
        }
    }

    pub fn descriptor(&self) -> String {
        match self {
            Class::Primitive(prim) => match prim {
                Primitive::Boolean => Descriptor::Boolean.descriptor(),
                Primitive::Byte => Descriptor::Byte.descriptor(),
                Primitive::Short => Descriptor::Short.descriptor(),
                Primitive::Char => Descriptor::Char.descriptor(),
                Primitive::Int => Descriptor::Int.descriptor(),
                Primitive::Long => Descriptor::Long.descriptor(),
                Primitive::Float => Descriptor::Float.descriptor(),
                Primitive::Double => Descriptor::Double.descriptor()
            }
            Class::Array { component } => format!("[{}", component.descriptor()),
            Class::Object { file } => format!("L{};", file.this_class)
        }
    }

    pub fn is_instance_of(&self, descriptor: &Descriptor) -> bool {
        match self {
            Class::Primitive(prim) => match prim {
                Primitive::Boolean => descriptor.eq(&Descriptor::Boolean),
                Primitive::Byte => descriptor.eq(&Descriptor::Byte),
                Primitive::Short => descriptor.eq(&Descriptor::Short),
                Primitive::Char => descriptor.eq(&Descriptor::Char),
                Primitive::Int => descriptor.eq(&Descriptor::Int),
                Primitive::Long => descriptor.eq(&Descriptor::Long),
                Primitive::Float => descriptor.eq(&Descriptor::Float),
                Primitive::Double => descriptor.eq(&Descriptor::Double)
            }
            Class::Array { component } => match descriptor {
                Descriptor::Array(inner) => component.is_instance_of(inner),
                _ => false
            }
            Class::Object { file } => file.is_instance_of(descriptor)
        }
    }

    pub fn unwrap_object_class(&self) -> &Rc<object::Class> {
        match self {
            Class::Object { file } => file,
            _ => panic!("error")
        }
    }
}
