use assert_cmd::Command;

#[test]
fn main_not_found() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .current_dir("java")
        .arg("ABC")
        .assert()
        .failure()
        .code(1)
        .stdout("")
        .stderr("Error: Could not find or load main class ABC");
}
