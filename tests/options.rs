use assert_cmd::Command;

#[test]
fn d32_fails() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-d32 IDoNotExist".split_whitespace())
        .assert()
        .failure()
        .code(1)
        .stdout("")
        .stderr("Error: This Java instance does not support a 32-bit JVM.
Please install the desired version.
");
}

#[test]
fn d64_silent() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-d64 PrintArgs a b".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("a\nb\n")
        .stderr("");
}

#[test]
fn help_question_mark() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-?".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr(predicates::str::starts_with("Usage: robusta [-options]"));
}

#[test]
fn help() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-help".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr(predicates::str::starts_with("Usage: robusta [-options]"));
}

#[test]
fn version() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-version IDoNotExist".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stderr(predicates::str::starts_with("robusta version \""))
        .stdout("");
}

#[test]
fn show_version() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-showversion PrintArgs a b".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stderr(predicates::str::starts_with("robusta version \""))
        .stdout("a\nb\n");
}

#[test]
fn unrecognized_option() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("-iDoNotExist".split_whitespace())
        .assert()
        .failure()
        .code(1)
        .stderr("Unrecognized option: -iDoNotExist\n")
        .stdout("");
}
