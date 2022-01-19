use assert_cmd::Command;

#[test]
fn empty_main() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .arg("EmptyMain")
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}
