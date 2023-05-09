extern crate assert_cmd;

use assert_cmd::Command;

#[test]
fn empty_main() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("EmptyMain")
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn hash_code() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("HashCode")
        .assert()
        .success()
        .code(0)
        .stdout("1313221257
-1557486435
-613450417
-2064311855
")
        .stderr("");
}

#[test]
fn wait_and_notify() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("WaitAndNotify")
        .assert()
        .success()
        .code(0)
        .stdout("First Message
Second Message
Waiting for notify
Notified!
")
        .stderr("");
}

#[test]
fn inheritance() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("Inheritance")
        .assert()
        .success()
        .code(0)
        .stdout("Inheritance$Animal.<clinit>
Inheritance$Dog.<clinit>
Inheritance$Cat.<clinit>
Woof, I'm a Whippet, and my name is Dave
I'm a cat, I'm too good for you!
Hello, I'm an animal and my name is Bird
")
        .stderr("");
}

#[test]
fn multi_threaded_oom() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("MultiThreadedOutOfMemory")
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn oom() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("OutOfMemory")
        .assert()
        .success()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn print_args() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .args("PrintArgs A B C D".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("A
B
C
D
")
        .stderr("");
}

#[test]
fn print_constants() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("PrintConstants")
        .assert()
        .success()
        .code(0)
        .stdout("hello world
542354326
")
        .stderr("");
}

#[test]
fn maths_instructions() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("MathsInstructions")
        .assert()
        .success()
        .code(0)
        .stdout("10
32766
32806
54235245
54268051
-2093215598
-2093269833
54213816
1943748040
-1943748040
-357832
-136
5432
546744
542560
17361920
542560
67820
1048574
-1048574
1048574
0
-1
0
1
2
3
4
5
-100
ᦜ
6556
5642652.0
5642652.0
5642652
5642652
").stderr("");
}

#[test]
fn throws_none() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .arg("Throws")
        .assert()
        .success()
        .code(0)
        .stdout("Starting main
Starting foo
Starting bar
Finally bar
Finishing bar
Finally foo
Finishing foo
Finally main
Finishing main
")
        .stderr("");
}

#[test]
fn throws_foo() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .args("Throws foo".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("Starting main
Starting foo
Caught illegal state exception in foo: throwing in foo
Finally foo
Caught illegal state exception in main: throwing in foo
Finally main
")
        .stderr("Exception in thread \"main\" java.lang.IllegalStateException: throwing in foo
	at Throws.foo(Throws.java:29)
	at Throws.main(Throws.java:20)
");
}

#[test]
fn throws_bar() {
    let mut robusta = Command::cargo_bin("robusta").unwrap();

    robusta
        .current_dir("../")
        .args("Throws bar".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout("Starting main
Starting foo
Starting bar
Caught runtime exception in bar: throwing in bar
Finally bar
Caught runtime exception in foo: throwing in bar
Finally foo
Caught runtime exception in main: throwing in bar
Finally main
")
        .stderr("Exception in thread \"main\" java.lang.RuntimeException: throwing in bar
	at Throws.bar(Throws.java:48)
	at Throws.foo(Throws.java:39)
	at Throws.main(Throws.java:20)
");
}