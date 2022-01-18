use assert_cmd::Command;

#[test]
fn empty_args() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "java")
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
        .env("ROBUSTA_CLASSPATH", "java")
        .args("PrintArgs hello".split_whitespace())
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
        .env("ROBUSTA_CLASSPATH", "java")
        .args("PrintArgs hello world how are you".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("hello\nworld\nhow\nare\nyou\n")
        .stderr("");
}
