use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use zip::ZipArchive;
use crate::class;
use crate::class_file;
use crate::class::{Class, Field};

use crate::class_file::{ClassFile, Reader};
use crate::descriptor::{Descriptor, MethodDescriptor};
use crate::heap::Value;

const ACC_NATIVE: u16 = 0x0100;
const ACC_STATIC: u16 = 0x0008;

pub struct ClassLoader {
    loaders: Vec<Box<dyn Loader>>,
    loaded: HashMap<String, Rc<Class>>,
    init: HashSet<String>,
    static_fields: HashMap<String, HashMap<u16, Value>>,
}

impl ClassLoader {
    pub fn new(class_path: &str) -> Self {
        let mut class_loader = ClassLoader {
            loaders: vec![],
            loaded: HashMap::new(),
            init: HashSet::new(),
            static_fields: HashMap::new(),
        };

        for path in std::env::split_paths(class_path) {
            if path.extension().map_or(false, |e| e.eq("jar")) {
                let jar_file = File::open(path.clone())
                    .expect(format!("Failed to open file {:?}", path.as_os_str()).as_str());
                let zip_arch = ZipArchive::new(jar_file).unwrap();
                let loader = JarLoader { jar: zip_arch };
                class_loader.loaders.push(Box::new(loader));
            } else {
                let loader = DirLoader { dir: path };
                class_loader.loaders.push(Box::new(loader));
            }
        }

        class_loader
    }

    pub fn load(&mut self, class: &str) -> Option<Rc<Class>> {
        if !self.loaded.contains_key(class) {
            let class_file = self.loaders.iter_mut()
                .map(|loader| loader.load(class))
                .find(|class| class.is_some())
                .expect(format!("Could not find class {}", class).as_str())
                .expect(format!("Could not find class {}", class).as_str());
            let class = self.class_from(&class_file);
            self.loaded.insert(class.this_class.clone(), class);
        }
        self.loaded.get(class).map(|class| class.clone())
    }

    pub fn uninit_parents(&self, class: &str) -> Vec<String> {
        let class = self.loaded.get(class).unwrap().clone();
        let mut parents: Vec<String> = class.parent_iter()
            .filter(|c| c.find_method("<clinit>", &MethodDescriptor::parse("()V")).is_some())
            .map(|c| c.this_class.clone())
            .filter(|c| !self.init.contains(c))
            .collect();
        parents.reverse();
        parents
    }

    pub fn init_parent(&mut self, class: &str) {
        self.init.insert(class.to_string());
    }

    pub fn get_static(&self, class: &str, idx: u16) -> Value {
        self.static_fields.get(class).unwrap().get(&idx).unwrap().clone()
    }

    pub fn put_static(&mut self, class: &str, idx: u16, value: Value) {
        self.static_fields.get_mut(class).unwrap().insert(idx, value);
    }

    fn class_from(&mut self, class_file: &ClassFile) -> Rc<Class> {
        let mut const_pool = HashMap::new();
        for (idx, con) in class_file.const_pool.iter() {
            let con = match con {
                class_file::Const::Class(class) => {
                    let class_file::Utf8 { bytes } = class_file.get_const(class.name_idx).expect_utf8();
                    let name = String::from_utf8(bytes.clone()).unwrap();
                    class::Const::Class(class::ClassRef { name })
                }
                class_file::Const::Int(int) => {
                    class::Const::Int(class::Integer { int: int.int })
                }
                class_file::Const::Float(float) => {
                    class::Const::Float(class::Float { float: float.float })
                }
                class_file::Const::Long(long) => {
                    class::Const::Long(class::Long { long: long.long })
                }
                class_file::Const::Double(double) => {
                    class::Const::Double(class::Double { double: double.double })
                }
                class_file::Const::String(string) => {
                    let class_file::Utf8 { bytes } = class_file.get_const(string.utf8_idx).expect_utf8();
                    class::Const::String(class::String { string: String::from_utf8(bytes.clone()).unwrap() })
                }
                class_file::Const::FieldRef(field_ref) => {
                    let class = class_file.get_const(field_ref.class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(field_ref.name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = String::from_utf8(class_name.bytes.clone()).unwrap();
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = String::from_utf8(descriptor.bytes.clone()).unwrap();

                    class::Const::Field(class::FieldRef { class: class_name, name, descriptor: Descriptor::parse(&descriptor) })
                }
                class_file::Const::MethodRef(method_ref) => {
                    let class = class_file.get_const(method_ref.class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(method_ref.name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = String::from_utf8(class_name.bytes.clone()).unwrap();
                    let name = String::from_utf8(name.bytes.clone()).unwrap();
                    let descriptor = String::from_utf8(descriptor.bytes.clone()).unwrap();

                    class::Const::Method(class::MethodRef { class: class_name, name, descriptor: MethodDescriptor::parse(&descriptor) })
                }
                _ => {
                    continue;
                }
            };
            const_pool.insert(idx.clone(), con);
        }

        let this_class = class_file.get_const(class_file.this_class).expect_class();
        let this_class_name = class_file.get_const(this_class.name_idx).expect_utf8();
        let this_class = String::from_utf8(this_class_name.bytes.clone()).unwrap();

        let super_class = Some(class_file.super_class)
            .filter(|idx| *idx != 0)
            .map(|idx| {
                let super_class = class_file.get_const(idx).expect_class();
                let super_class_name = class_file.get_const(super_class.name_idx).expect_utf8();
                let super_class_name = String::from_utf8(super_class_name.bytes.clone()).unwrap();
                self.load(&super_class_name).expect(&format!("Could not load class {}", &super_class_name))
            });

        let interfaces = class_file.interfaces.iter().map(|idx| {
            let interface = class_file.get_const(idx.clone()).expect_class();
            let interface_name = class_file.get_const(interface.name_idx).expect_utf8();
            String::from_utf8(interface_name.bytes.clone()).unwrap()
        }).collect();

        let fields: Vec<Rc<Field>> = class_file.fields.iter()
            .map(|field| {
                let name = class_file.get_const(field.name_idx).expect_utf8();
                let descriptor = class_file.get_const(field.descriptor_idx).expect_utf8();
                Rc::new(class::Field {
                    name: String::from_utf8(name.bytes.clone()).unwrap(),
                    descriptor: Descriptor::parse(
                        String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()),
                    access_flags: field.access_flags,
                })
            }).collect();

        let static_fields = fields.iter()
            .enumerate()
            .filter(|(_, f)| f.access_flags & ACC_STATIC != 0)
            .map(|(idx, f)| {
                let value = f.descriptor.zero_value();
                (idx as u16, value)
            }).collect();

        self.static_fields.insert(this_class.clone(), static_fields);

        let methods = class_file.methods.iter().map(|method| {
            let name = class_file.get_const(method.name_idx).expect_utf8();
            let descriptor = class_file.get_const(method.descriptor_idx).expect_utf8();
            let native = (method.access_flags & ACC_NATIVE) != 0;
            let mut max_locals = 0;
            let mut max_stack = 0;
            let code = if native { vec![] } else {
                let code = method.attributes.iter()
                    .find_map(|attr| match attr {
                        class_file::Attribute::Code(code) => Some(code),
                        _ => None
                    }).unwrap();
                max_locals = code.max_locals;
                max_stack = code.max_stack;
                code.code.clone()
            };
            Rc::new(class::Method {
                name: String::from_utf8(name.bytes.clone()).unwrap(),
                descriptor: MethodDescriptor::parse(
                    String::from_utf8(descriptor.bytes.clone()).unwrap().as_str()),
                native,
                max_locals,
                max_stack,
                code,
            })
        }).collect();

        Rc::from(Class {
            minor_version: class_file.minor_version,
            major_version: class_file.major_version,
            const_pool,
            access_flags: class_file.access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
        })
    }
}

trait Loader {
    fn load(&mut self, class_name: &str) -> Option<ClassFile>;
}

struct DirLoader {
    dir: PathBuf,
}

impl Loader for DirLoader {
    fn load(&mut self, class_name: &str) -> Option<ClassFile> {
        let file_name = self.dir
            .join(class_name)
            .with_extension("class");

        let file = File::open(file_name);

        file.ok().map(|mut file| Reader::new(&mut file).read_class_file())
    }
}

struct JarLoader {
    jar: ZipArchive<File>,
}

impl Loader for JarLoader {
    fn load(&mut self, class_name: &str) -> Option<ClassFile> {
        let file_name = Path::new(class_name).with_extension("class");

        let zip_file = self.jar.by_name(file_name.to_str().unwrap());

        zip_file.ok().map(|mut zip_file| Reader::new(&mut zip_file).read_class_file())
    }
}
