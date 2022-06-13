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
    Object { file: object::Class },
}

impl Class {
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
}
