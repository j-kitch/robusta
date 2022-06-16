use assert_cmd::Command;

#[test]
fn instance_of() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .arg("InstanceOf")
        .assert()
        .success()
        .code(0)
        .stdout("Object: true
Object Array: false
String: false
String Array: false
InstanceOf: false
InstanceOf Array: false
Object: true
Object Array: false
String: true
String Array: false
InstanceOf: false
InstanceOf Array: false
Object: true
Object Array: false
String: false
String Array: false
InstanceOf: true
InstanceOf Array: false
")
        .stderr("");
}
