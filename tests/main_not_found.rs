use assert_cmd::Command;

#[test]
fn main_not_found() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "java")
        .arg("ABC")
        .assert()
        .failure();
}
