use assert_cmd::Command;

#[test]
fn empty_main() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .current_dir("java")
        .arg("EmptyMain")
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}
