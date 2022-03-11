use assert_cmd::Command;

#[test]
fn english() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("StringConcat Steph 0".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("Steph, welcome\n")
        .stderr("");
}

#[test]
fn german() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("StringConcat Helmut 1".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("Helmut, wilkommen\n")
        .stderr("");
}
