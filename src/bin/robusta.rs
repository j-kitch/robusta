use std::{env, format};
use std::fs::File;
use std::io::Read;

fn main() {
    let main_class_name = env::args().nth(1).unwrap();
    let main_class = format!("{}.class", &main_class_name);

    let main_class_file = File::open(&main_class);
    let mut main_class_file = match main_class_file {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Could not find or load main class {}", &main_class_name);
            std::process::exit(1);
        }
    };

    let mut class_file = ClassFile {
        minor_version: 0,
        major_version: 0,
        const_pool: vec![],
        access_flags: 0,
        this_class: 0,
        super_class: 0,
        interfaces: vec![],
        methods: vec![],
        attributes: vec![],
    };
    let mut four_bs: [u8; 4] = [0; 4];
    let mut two_bs: [u8; 2] = [0; 2];
    let mut one_bs: [u8; 1] = [0];

    main_class_file.read_exact(&mut four_bs).unwrap();

    main_class_file.read_exact(&mut two_bs).unwrap();
    class_file.minor_version = u16::from_be_bytes(two_bs);

    main_class_file.read_exact(&mut two_bs).unwrap();
    class_file.major_version = u16::from_be_bytes(two_bs);

    main_class_file.read_exact(&mut two_bs).unwrap();
    let const_pool_len = u16::from_be_bytes(two_bs);

    for _ in 0..(const_pool_len - 1) {
        main_class_file.read_exact(&mut one_bs).unwrap();
        let tag = one_bs[0];

        match tag {
            1 => {
                main_class_file.read_exact(&mut two_bs).unwrap();
                let length = u16::from_be_bytes(two_bs);
                let mut utf8 = Utf8 { bytes: vec![0; length as usize] };
                main_class_file.read_exact(&mut utf8.bytes).unwrap();

                class_file.const_pool.push(Const::Utf8(utf8));
            }
            7 => {
                let mut class = Class { name_idx: 0 };
                main_class_file.read_exact(&mut two_bs).unwrap();
                class.name_idx = u16::from_be_bytes(two_bs);

                class_file.const_pool.push(Const::Class(class));
            }
            10 => {
                let mut method_ref = MethodRef { class_idx: 0, name_and_type_idx: 0 };
                main_class_file.read_exact(&mut two_bs).unwrap();
                method_ref.class_idx = u16::from_be_bytes(two_bs);

                main_class_file.read_exact(&mut two_bs).unwrap();
                method_ref.name_and_type_idx = u16::from_be_bytes(two_bs);

                class_file.const_pool.push(Const::MethodRef(method_ref));
            }
            12 => {
                let mut name_and_type = NameAndType { name_idx: 0, descriptor_idx: 0 };

                main_class_file.read_exact(&mut two_bs).unwrap();
                name_and_type.name_idx = u16::from_be_bytes(two_bs);

                main_class_file.read_exact(&mut two_bs).unwrap();
                name_and_type.descriptor_idx = u16::from_be_bytes(two_bs);

                class_file.const_pool.push(Const::NameAndType(name_and_type));
            }
            _ => {
                panic!("Unknown tag {}", tag);
            }
        }
    }

    main_class_file.read_exact(&mut two_bs).unwrap();
    class_file.access_flags = u16::from_be_bytes(two_bs);

    main_class_file.read_exact(&mut two_bs).unwrap();
    class_file.this_class = u16::from_be_bytes(two_bs);

    main_class_file.read_exact(&mut two_bs).unwrap();
    class_file.super_class = u16::from_be_bytes(two_bs);

    main_class_file.read_exact(&mut two_bs).unwrap();
    let interfaces_len = u16::from_be_bytes(two_bs);

    for _ in 0..interfaces_len {
        main_class_file.read_exact(&mut two_bs).unwrap();
        let interface = u16::from_be_bytes(two_bs);
        class_file.interfaces.push(interface);
    }

    main_class_file.read_exact(&mut two_bs).unwrap();
    let fields_len = u16::from_be_bytes(two_bs);

    for _ in 0..fields_len {
        panic!("error")
    }

    main_class_file.read_exact(&mut two_bs).unwrap();
    let methods_len = u16::from_be_bytes(two_bs);

    for _ in 0..methods_len {
        main_class_file.read_exact(&mut two_bs).unwrap();
        let access_flags = u16::from_be_bytes(two_bs);

        main_class_file.read_exact(&mut two_bs).unwrap();
        let name_idx = u16::from_be_bytes(two_bs);

        main_class_file.read_exact(&mut two_bs).unwrap();
        let desc_idx = u16::from_be_bytes(two_bs);

        main_class_file.read_exact(&mut two_bs).unwrap();
        let attr_count = u16::from_be_bytes(two_bs);

        let mut method = Method {
            access_flags,
            name_idx,
            descriptor_idx: desc_idx,
            attributes: vec![],
        };

        for _ in 0..attr_count {
            main_class_file.read_exact(&mut two_bs).unwrap();
            let name_idx = u16::from_be_bytes(two_bs);

            main_class_file.read_exact(&mut four_bs).unwrap();
            let attr_len = u32::from_be_bytes(four_bs);

            let mut bytes = vec![0; attr_len as usize];
            main_class_file.read_exact(&mut bytes).unwrap();

            method.attributes.push(Attribute {
                name_idx,
                bytes,
            });
        }

        class_file.methods.push(method);
    }

    main_class_file.read_exact(&mut two_bs).unwrap();
    let attr_count = u16::from_be_bytes(two_bs);

    for _ in 0..attr_count {
        main_class_file.read_exact(&mut two_bs).unwrap();
        let name_idx = u16::from_be_bytes(two_bs);

        main_class_file.read_exact(&mut four_bs).unwrap();
        let attr_len = u32::from_be_bytes(four_bs);

        let mut bytes = vec![0; attr_len as usize];
        main_class_file.read_exact(&mut bytes).unwrap();

        class_file.attributes.push(Attribute {
            name_idx,
            bytes,
        });
    }

    println!("{:?}", class_file);
}

#[derive(Debug)]
struct ClassFile {
    minor_version: u16,
    major_version: u16,
    const_pool: Vec<Const>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct Utf8 {
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct Class {
    name_idx: u16,
}

#[derive(Debug)]
struct MethodRef {
    class_idx: u16,
    name_and_type_idx: u16,
}

#[derive(Debug)]
struct NameAndType {
    name_idx: u16,
    descriptor_idx: u16,
}

#[derive(Debug)]
enum Const {
    Utf8(Utf8),
    Class(Class),
    MethodRef(MethodRef),
    NameAndType(NameAndType),
}

#[derive(Debug)]
struct Method {
    access_flags: u16,
    name_idx: u16,
    descriptor_idx: u16,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct Attribute {
    name_idx: u16,
    bytes: Vec<u8>,
}
