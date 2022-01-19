use assert_cmd::Command;

#[test]
fn return_values() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .arg("ReturnValues")
        .assert()
        .success()
        .code(0)
        .stdout("10\n100\n10.2\n123.456789\nhello world\n")
        .stderr("");
}
