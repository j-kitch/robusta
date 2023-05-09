use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

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
    files: HashMap<PathBuf, Vec<u8>>
}

impl JarLoader {
    pub fn new(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let mut archive = ZipArchive::new(BufReader::new(file)).unwrap();

        let size = archive.len();
        let mut files = HashMap::with_capacity(size);

        for idx in 0..size {
            let mut file = archive.by_index(idx).unwrap();

            let file_name = file.enclosed_name().unwrap().to_owned();

            let mut data = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut data).unwrap();

            files.insert(file_name, data);
        }

        JarLoader { files }
    }
}

impl Loader for JarLoader {
    fn find(&self, class_name: &str) -> Option<ClassFile> {
        let file_name = PathBuf::from(class_name.replace(".", "/"))
            .with_extension("class");

        let data = self.files.get(&file_name);
        if data.is_none() {
            return None;
        }

        let mut bytes = data.unwrap().as_slice();
        Some(parse(&mut bytes))
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
                    Box::new(JarLoader::new(path)) as _
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
