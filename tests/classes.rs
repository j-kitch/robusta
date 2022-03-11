use assert_cmd::Command;

#[test]
fn classes() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("Classes".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("java.lang.String\njava.lang.Integer\njava.lang.Object\nClasses\n")
        .stderr("");
}
