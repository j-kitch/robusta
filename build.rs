use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let java_libraries = ["robusta-java-runtime", "robusta-java-test"];

    for java_library in java_libraries {
        build_java_library(Path::new(java_library));
    }
}

fn build_java_library(path: &Path) {
    if cfg!(debug_assertions) {
        // Conditionally rebuild only if we have changed src, pom or target dir.
        let to_watch = [
            PathBuf::from("src"),
            PathBuf::from("pom.xml"),
            PathBuf::from("target").join(path.with_extension("jar")),
        ];

        for child in to_watch {
            println!("cargo:rerun-if-changed={}", path.join(child).to_str().unwrap());
        }
    }

    let args = if cfg!(debug_assertions) { "install" } else { "clean install" };

    Command::new("mvn")
        .current_dir(path)
        .args(args.split_whitespace())
        .status()
        .unwrap();
}
