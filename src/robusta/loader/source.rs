use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use zip::ZipArchive;
use crate::class_file::ClassFile;
use crate::loader::parser::parse;

/// The raw bytes of a class file, read from a source in the class path.
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(vec: Vec<u8>) -> Self {
        Bytes(vec)
    }

    pub fn get_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn get_reader<'a>(&'a self) -> Box<dyn Read + 'a> {
        Box::new(self.0.as_slice())
    }
}

impl Bytes {
    /// Read the entire contents from the reader into the Bytes object.
    fn from_reader<R: Read>(reader: R) -> Self {
        let mut reader = BufReader::new(reader);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        Bytes(buffer)
    }
}

/// A source of class files.
pub trait Source: Send + Sync {
    /// Find the class file matching the given name, and return the underlying bytes.
    fn find(&self, class_name: &str) -> Option<ClassFile>;
}

/// A directory source, looking for class files in a given directory.
struct DirSource {
    /// The root directory to search for class files from.
    root_dir: PathBuf,
}

impl Source for DirSource {
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

/// A jar file source, looking for class files within a jar file.
struct JarSource {
    jar_path: PathBuf,
}

impl Source for JarSource {
    fn find(&self, class_name: &str) -> Option<ClassFile> {
        let zip_file = File::open(&self.jar_path).ok();
        let mut zip_arch = zip_file.and_then(|file| {
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

/// An ordered combination of sources, delegating to each inner source.
pub struct Sources {
    sources: Vec<Box<dyn Source>>,
}

impl Sources {
    /// Construct a new set of sources from the class path.
    pub fn new(class_path: Vec<PathBuf>) -> Self {
        Sources {
            sources: class_path.iter().map(|path| {
                if path.is_dir() {
                    Box::new(DirSource { root_dir: path.clone() }) as _
                } else if path.extension().unwrap().eq(&PathBuf::from("jar")) {
                    Box::new(JarSource { jar_path: path.clone() }) as _
                } else {
                    panic!("Unknown type of path {}", path.to_str().unwrap())
                }
            }).collect()
        }
    }
}

impl Source for Sources {
    fn find(&self, class_name: &str) -> Option<ClassFile> {
        self.sources.iter()
            .flat_map(|source| source.find(class_name))
            .next()
    }
}

//
// #[cfg(test)]
// mod tests {
//     use std::path::Path;
//     use super::*;
//
//     #[test]
//     fn dir_source() {
//         let mut expected = Vec::new();
//         File::open("./classes/EmptyMain.class").unwrap().read_to_end(&mut expected).unwrap();
//
//         let mut dir_source = DirSource { root_dir: Path::new("./classes").to_path_buf() };
//         let result = dir_source.find("EmptyMain").unwrap();
//
//         assert_eq!(expected, result.0);
//     }
//
//     #[test]
//     fn jar_source() {
//         let mut expected = Vec::new();
//         File::open("./classes/EmptyMain.class").unwrap().read_to_end(&mut expected).unwrap();
//
//         let mut jar_source = JarSource { jar_path: Path::new("./classes/EmptyMain.jar").to_path_buf() };
//         let result = jar_source.find("EmptyMain").unwrap();
//
//         assert_eq!(expected, result.0);
//     }
//
//     #[test]
//     fn sources() {
//         let mut expected = Vec::new();
//         File::open("./classes/EmptyMain.class").unwrap().read_to_end(&mut expected).unwrap();
//
//         let mut source = Sources::new(vec![
//             PathBuf::from("./classes"),
//             PathBuf::from("./classes/EmptyMain.jar")
//         ]);
//         let result = source.find("EmptyMain").unwrap();
//
//         assert_eq!(expected, result.0);
//     }
// }