use std::path::Path;
use std::process::Command;

fn main() {
    build_java_library(Path::new("robusta-java-runtime"));
    build_java_library(Path::new("robusta-java-test"));
}

fn build_java_library(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    Command::new("mvn")
        .current_dir(path)
        .args("clean install".split_whitespace())
        .status()
        .unwrap();
}
