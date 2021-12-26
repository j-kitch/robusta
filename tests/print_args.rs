use assert_cmd::Command;

#[test]
fn empty_args() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .current_dir("java")
        .arg("PrintArgs")
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn one_arg() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .current_dir("java")
        .arg("PrintArgs hello")
        .assert()
        .success()
        .code(0)
        .stdout("hello\n")
        .stderr("");
}

#[test]
fn multiple_args() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .current_dir("java")
        .arg("PrintArgs hello world how are you")
        .assert()
        .success()
        .code(0)
        .stdout("hello\nworld\nhow\nare\nyou\n")
        .stderr("");
}
