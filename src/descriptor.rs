use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Descriptor {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Object(String),
    Array(Box<Descriptor>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodDescriptor {
    pub args: Vec<Descriptor>,
    pub returns: Option<Descriptor>,
}

impl Descriptor {
    pub fn parse(descriptor: &str) -> Descriptor {
        let (rest, desc) = parse_descriptor(descriptor);
        if rest.len() > 0 {
            panic!("invalid descriptor {}", descriptor);
        }
        desc
    }

    pub fn descriptor(&self) -> String {
        match self {
            Descriptor::Boolean => String::from("Z"),
            Descriptor::Byte => String::from("B"),
            Descriptor::Char => String::from("C"),
            Descriptor::Short => String::from("S"),
            Descriptor::Int => String::from("I"),
            Descriptor::Long => String::from("J"),
            Descriptor::Float => String::from("F"),
            Descriptor::Double => String::from("D"),
            Descriptor::Object(class_name) => format!("L{};", class_name),
            Descriptor::Array(component) => format!("[{}", component.descriptor()),
        }
    }

    pub fn category(&self) -> usize {
        match self {
            Descriptor::Double | Descriptor::Long => 2,
            _ => 1,
        }
    }
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.descriptor())
    }
}

impl MethodDescriptor {
    pub fn parse(descriptor: &str) -> MethodDescriptor {
        let mut method_descriptor = MethodDescriptor { args: vec![], returns: None };
        if "(".ne(&descriptor[..1]) {
            panic!("Invalid method descriptor {}", descriptor);
        }
        let mut descriptor = &descriptor[1..];
        while ")".ne(&descriptor[..1]) {
            let (rest, param) = parse_descriptor(descriptor);
            method_descriptor.args.push(param);
            descriptor = rest;
        }
        if ")".ne(&descriptor[..1]) {
            panic!("Invalid method descriptor {}", descriptor);
        }
        descriptor = &descriptor[1..];
        if descriptor.ne("V") {
            let (rest, returns) = parse_descriptor(descriptor);
            if rest.len() > 0 {
                panic!("Invalid method descriptor {}", descriptor);
            }
            method_descriptor.returns = Some(returns);
        }
        method_descriptor
    }

    pub fn descriptor(&self) -> String {
        let arg_parts: String = self.args.iter().map(|d| d.descriptor()).collect();
        let returns = self.returns.as_ref().map_or(String::from("V"), |d| d.descriptor());
        format!("({}){}", arg_parts, returns)
    }

    pub fn category(&self) -> usize {
        self.args.iter()
            .map(|a| a.category())
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

impl Display for MethodDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.descriptor())
    }
}

fn parse_descriptor(input: &str) -> (&str, Descriptor) {
    match &input[..1] {
        "Z" => (&input[1..], Descriptor::Boolean),
        "B" => (&input[1..], Descriptor::Byte),
        "C" => (&input[1..], Descriptor::Char),
        "S" => (&input[1..], Descriptor::Short),
        "I" => (&input[1..], Descriptor::Int),
        "J" => (&input[1..], Descriptor::Long),
        "F" => (&input[1..], Descriptor::Float),
        "D" => (&input[1..], Descriptor::Double),
        "L" => {
            let end = input.find(';').unwrap();
            let class_name = String::from(&input[1..end]);
            (&input[end + 1..], Descriptor::Object(class_name))
        }
        "[" => {
            let (rest, component) = parse_descriptor(&input[1..]);
            (rest, Descriptor::Array(Box::new(component)))
        }
        _ => panic!("cannot parse descriptor {}", input)
    }
}
