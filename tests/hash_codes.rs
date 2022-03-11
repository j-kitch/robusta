use assert_cmd::Command;

#[test]
fn english() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("HashCodes".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("1658361822\n562119498\n1444506296\n")
        .stderr("");
}
