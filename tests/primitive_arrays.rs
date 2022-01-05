use assert_cmd::Command;

#[test]
fn primitive_arrays() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .current_dir("java")
        .arg("PrimitiveArrays")
        .assert()
        .success()
        .code(0)
        .stdout("true\n1\n\x03\n0\n4312\n-4321553\n-234.567\n-4321.54322\n")
        .stderr("");
}