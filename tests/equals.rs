use assert_cmd::Command;

#[test]
fn classes() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("Equals abcd".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("objectA == objectA: true
objectA == objectB: false
objectB == objectA: false
objectB == objectB: true
objectA == stringA: false
objectA == stringB: false
objectB == stringA: false
objectB == stringB: false
stringA == objectA: false
stringA == objectB: false
stringB == objectA: false
stringB == objectB: false
stringA == stringA: true
stringA == stringB: false
stringB == stringA: false
stringB == stringB: true
stringA == stringA: true
stringA == stringB: false
stringB == stringA: false
stringB == stringB: true
")
        .stderr("");
}

#[test]
fn classes_equal_strs() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args(["Equals", "hello world"])
        .assert()
        .success()
        .code(0)
        .stdout("objectA == objectA: true
objectA == objectB: false
objectB == objectA: false
objectB == objectB: true
objectA == stringA: false
objectA == stringB: false
objectB == stringA: false
objectB == stringB: false
stringA == objectA: false
stringA == objectB: false
stringB == objectA: false
stringB == objectB: false
stringA == stringA: true
stringA == stringB: true
stringB == stringA: true
stringB == stringB: true
stringA == stringA: true
stringA == stringB: true
stringB == stringA: true
stringB == stringB: true
")
        .stderr("");
}
