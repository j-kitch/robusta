use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

/// A `Source` is a source that a class file can be read from.
pub trait Source {
    /// Attempt to get a reader of a class file.
    ///
    /// If the class file couldn't be found, `None` is returned.
    fn open<'a>(&'a mut self, class_name: &str) -> Option<Box<dyn Read + 'a>>;
}

/// Create a new class loader source from the class path.
pub fn new_source(class_path: Vec<PathBuf>) -> Box<dyn Source + Sync + Send> {
    let mut sources = Vec::new();
    for path in class_path.iter() {
        let source: Box<dyn Source + Sync + Send> = if path.is_dir() {
            Box::new(DirSource { root_dir: path.to_path_buf() })
        } else {
            let file = File::open(path).unwrap();
            let zip = ZipArchive::new(file).unwrap();
            Box::new(JarSource { jar: zip })
        };
        sources.push(source);
    }
    Box::new(CompositeSource { sources })
}

/// Create a composite source from other sources.
struct CompositeSource {
    sources: Vec<Box<dyn Source + Sync + Send>>,
}

impl Source for CompositeSource {
    /// Iterate through the sources, until we find our reader.
    fn open<'a>(&'a mut self, class_name: &str) -> Option<Box<dyn Read + 'a>> {
        self.sources.iter_mut().flat_map(|s| s.open(class_name)).next()
    }
}

/// A directory source, looking for class files from a given directory.
struct DirSource {
    /// The root directory to start searching from.
    root_dir: PathBuf,
}

impl Source for DirSource {
    fn open<'a>(&'a mut self, class_name: &str) -> Option<Box<dyn Read + 'a>> {
        let path = self.root_dir
            .join(class_name.replace(".", "/"))
            .with_extension("class");

        File::open(path).ok().map(|f| Box::new(f) as _)
    }
}

/// A jar source, looking for class files in a jar file.
struct JarSource {
    jar: ZipArchive<File>,
}

impl Source for JarSource {
    fn open<'a>(&'a mut self, class_name: &str) -> Option<Box<dyn Read + 'a>> {
        let path = Path::new(class_name.replace(".", "/").as_str())
            .with_extension("class");

        self.jar.by_name(path.to_str().unwrap()).ok().map(|f| Box::new(f) as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_source() {
        let mut expected = Vec::new();
        let mut read = Vec::new();
        File::open("./classes/EmptyMain.class").unwrap().read_to_end(&mut expected).unwrap();

        let mut dir_source = DirSource { root_dir: Path::new("./classes").to_path_buf() };
        dir_source.open("EmptyMain").unwrap().read_to_end(&mut read).unwrap();

        assert_eq!(expected, read);
    }

    #[test]
    fn jar_source() {
        let mut expected = Vec::new();
        let mut read = Vec::new();
        File::open("./classes/EmptyMain.class").unwrap().read_to_end(&mut expected).unwrap();

        let mut jar_source = JarSource { jar: ZipArchive::new(File::open("./classes/EmptyMain.jar").unwrap()).unwrap() };
        jar_source.open("EmptyMain").unwrap().read_to_end(&mut read).unwrap();

        assert_eq!(expected, read);
    }

    #[test]
    fn new_source_open() {
        let mut expected = Vec::new();
        let mut read = Vec::new();
        File::open("./classes/EmptyMain.class").unwrap().read_to_end(&mut expected).unwrap();

        let mut sources = new_source(vec![
            PathBuf::from("./classes/EmptyMain.jar"),
            PathBuf::from("./classes"),
        ]);
        sources.open("EmptyMain").unwrap().read_to_end(&mut read).unwrap();

        assert_eq!(expected, read);
    }
}