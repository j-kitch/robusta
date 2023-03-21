use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use zip::ZipArchive;

use crate::class_file::ClassFile;
use crate::loader::parser::parse;

mod parser;

/// A class file loader.
pub trait Loader: Send + Sync {
    /// Find the class file matching the given name and return it.
    fn find(&self, class_name: &str) -> Option<ClassFile>;
}

/// A directory loader, looking for class files in a given directory.
struct DirLoader {
    /// The root directory to search for class files from.
    root_dir: PathBuf,
}

impl Loader for DirLoader {
    fn find(&self, class_name: &str) -> Option<ClassFile> {
        let file_path = self.root_dir
            .join(class_name.replace(".", "/"))
            .with_extension("class");

        let mut file = File::open(file_path).ok();

        file.as_mut().map(|file| {
            parse(file)
        })
    }
}

/// A jar file loader, looking for class files within a jar file.
struct JarLoader {
    jar_path: PathBuf,
}

impl Loader for JarLoader {
    fn find(&self, class_name: &str) -> Option<ClassFile> {
        let zip_file = File::open(&self.jar_path).ok();
        let zip_arch = zip_file.and_then(|file| {
            ZipArchive::new(BufReader::new(file)).ok()
        });

        let file_name = PathBuf::from(class_name.replace(".", "/"))
            .with_extension("class");

        if zip_arch.is_none() {
            return None;
        }
        let mut zip_arch = zip_arch.unwrap();
        let zip_file = zip_arch.by_name(file_name.to_str().unwrap()).ok();
        if zip_file.is_none() {
            return None;
        }
        let mut zip_file = zip_file.unwrap();
        Some(parse(&mut zip_file))
    }
}

/// The class file loader delegates to each internal loader, looking for a matching
/// class file.
pub struct ClassFileLoader {
    loaders: Vec<Box<dyn Loader>>,
}

impl ClassFileLoader {
    /// Construct a new class file loader from the class path.
    pub fn new(class_path: Vec<PathBuf>) -> Self {
        ClassFileLoader {
            loaders: class_path.iter().map(|path| {
                if path.is_dir() {
                    Box::new(DirLoader { root_dir: path.clone() }) as _
                } else if path.extension().unwrap().eq(&PathBuf::from("jar")) {
                    Box::new(JarLoader { jar_path: path.clone() }) as _
                } else {
                    panic!("Unknown type of path {}", path.to_str().unwrap())
                }
            }).collect()
        }
    }
}

impl Loader for ClassFileLoader {
    fn find(&self, class_name: &str) -> Option<ClassFile> {
        self.loaders.iter()
            .flat_map(|source| source.find(class_name))
            .next()
    }
}
