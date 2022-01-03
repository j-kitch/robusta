use assert_cmd::Command;

#[test]
fn less_than() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .args("Branching 1 2".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("i1 and i2 are not equal\ni1 is less than i2\ni1 is less than or equal to i2\n")
        .stderr("");
}

#[test]
fn equal() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .args("Branching 2 2".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("i1 and i2 are equal\ni1 is less than or equal to i2\ni1 is greater than or equal to i2\n")
        .stderr("");
}

#[test]
fn greater_than() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .args("Branching 2 1".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("i1 and i2 are not equal\ni1 is greater than i2\ni1 is greater than or equal to i2\n")
        .stderr("");
}
