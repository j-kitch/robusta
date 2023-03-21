extern crate assert_cmd;

use assert_cmd::Command;

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