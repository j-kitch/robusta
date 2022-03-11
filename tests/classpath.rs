use assert_cmd::Command;

#[test]
fn cp_classpath() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .args("-cp robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar EmptyMain".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn classpath_classpath() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .args("-classpath robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar EmptyMain".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}

