use assert_cmd::Command;

#[test]
fn interfaces() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("Interfaces".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("i1.compareTo(i1) = 0
i1.compareTo(i2) = -1
i2.compareTo(i1) = 1
i2.compareTo(i2) = 0
")
        .stderr("");
}
