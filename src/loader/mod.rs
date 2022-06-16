use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use zip::ZipArchive;

use crate::descriptor::{Descriptor, MethodDescriptor};
use crate::heap::Value;
use crate::robusta::class::Class;
use crate::robusta::class::object;
use crate::robusta::class::object::Field;
use crate::robusta::class_file::{attribute, const_pool};
use crate::robusta::class_file::{ClassFile, Reader};

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

    pub fn load(&mut self, class_name: &str) -> Option<Rc<Class>> {
        if !self.loaded.contains_key(class_name) {
            let class_file = self.loaders.iter_mut()
                .map(|loader| loader.load(class_name))
                .find(|class| class.is_some())
                .expect(format!("Could not find class {}", class_name).as_str())
                .expect(format!("Could not find class {}", class_name).as_str());
            let class = self.class_from(&class_file);
            self.loaded.insert(class_name.to_string(), class);
        }
        self.loaded.get(class_name).map(|class| class.clone())
    }

    pub fn uninit_parents(&self, class: &str) -> Vec<String> {
        let class = self.loaded.get(class).unwrap().clone();
        if let Class::Object { file } = class.deref() {
            let mut parents: Vec<String> = file.parent_iter()
                .filter(|c| c.find_method("<clinit>", &MethodDescriptor::parse("()V")).is_some())
                .map(|c| c.this_class.clone())
                .filter(|c| !self.init.contains(c))
                .collect();
            parents.reverse();
            parents
        } else {
            Vec::new()
        }
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
                const_pool::Const::Class(const_pool::Class { name_idx }) => {
                    let name = class_file.get_const(*name_idx).expect_utf8();
                    object::Const::Class(object::ClassRef { name: name.utf8.clone() })
                }
                const_pool::Const::Integer(const_pool::Integer { int }) => {
                    object::Const::Int(object::Integer { int: *int })
                }
                const_pool::Const::Float(const_pool::Float { float }) => {
                    object::Const::Float(object::Float { float: *float })
                }
                const_pool::Const::Long(const_pool::Long { long }) => {
                    object::Const::Long(object::Long { long: *long })
                }
                const_pool::Const::Double(const_pool::Double { double }) => {
                    object::Const::Double(object::Double { double: *double })
                }
                const_pool::Const::String(const_pool::String { string_idx }) => {
                    let string = class_file.get_const(*string_idx).expect_utf8();
                    object::Const::String(object::String { string: string.utf8.to_string() })
                }
                const_pool::Const::Field(const_pool::Field { class_idx, name_and_type_idx }) => {
                    let class = class_file.get_const(*class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(*name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = class_name.utf8.clone();
                    let name = name.utf8.clone();
                    let descriptor = descriptor.utf8.clone();

                    object::Const::Field(object::FieldRef { class: class_name, name, descriptor: Descriptor::parse(&descriptor) })
                }
                const_pool::Const::Method(const_pool::Method { class_idx, name_and_type_idx }) => {
                    let class = class_file.get_const(*class_idx).expect_class();
                    let class_name = class_file.get_const(class.name_idx).expect_utf8();

                    let name_and_type = class_file.get_const(*name_and_type_idx).expect_name_and_type();
                    let name = class_file.get_const(name_and_type.name_idx).expect_utf8();
                    let descriptor = class_file.get_const(name_and_type.descriptor_idx).expect_utf8();

                    let class_name = class_name.utf8.clone();
                    let name = name.utf8.clone();
                    let descriptor = descriptor.utf8.clone();

                    object::Const::Method(object::MethodRef { class: class_name, name, descriptor: MethodDescriptor::parse(&descriptor) })
                }
                _ => {
                    continue;
                }
            };
            const_pool.insert(idx.clone(), con);
        }

        let this_class = class_file.get_const(class_file.this_class).expect_class();
        let this_class_name = class_file.get_const(this_class.name_idx).expect_utf8();
        let this_class = this_class_name.utf8.clone();

        let super_class = Some(class_file.super_class)
            .filter(|idx| *idx != 0)
            .map(|idx| {
                let super_class = class_file.get_const(idx).expect_class();
                let super_class_name = class_file.get_const(super_class.name_idx).expect_utf8();
                let super_class_name = super_class_name.utf8.clone();
                self.load(&super_class_name).expect(&format!("Could not load class {}", &super_class_name))
                    .unwrap_object_class().clone()
            });

        let interfaces = class_file.interfaces.iter().map(|idx| {
            let interface = class_file.get_const(idx.clone()).expect_class();
            let interface_name = class_file.get_const(interface.name_idx).expect_utf8();
            interface_name.utf8.clone()
        }).collect();

        let fields: Vec<Rc<Field>> = class_file.fields.iter()
            .map(|field| {
                let name = class_file.get_const(field.name_idx).expect_utf8();
                let descriptor = class_file.get_const(field.descriptor_idx).expect_utf8();
                Rc::new(object::Field {
                    name: name.utf8.clone(),
                    descriptor: Descriptor::parse(descriptor.utf8.as_str()),
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
                        attribute::Attribute::Code(code) => Some(code),
                        _ => None
                    }).unwrap();
                max_locals = code.max_locals;
                max_stack = code.max_stack;
                code.code.clone()
            };
            Rc::new(object::Method {
                name: name.utf8.clone(),
                descriptor: MethodDescriptor::parse(descriptor.utf8.as_str()),
                native,
                max_locals,
                max_stack,
                code,
            })
        }).collect();

        Rc::from(Class::Object { file: Rc::new(object::Class {
            version: class_file.version.clone(),
            const_pool,
            access_flags: class_file.access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
        }) })
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

        file.and_then(|mut file| Reader::new(BufReader::new(&mut file)).read_class_file()).ok()
    }
}

struct JarLoader {
    jar: ZipArchive<File>,
}

impl Loader for JarLoader {
    fn load(&mut self, class_name: &str) -> Option<ClassFile> {
        let file_name = Path::new(class_name).with_extension("class");

        let zip_file = self.jar.by_name(file_name.to_str().unwrap());
        if zip_file.is_err() {
            return None;
        }

        Some(zip_file.map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            .and_then(|mut zip_file| Reader::new(BufReader::new(
                &mut zip_file
            )).read_class_file())
            .unwrap())
    }
}
