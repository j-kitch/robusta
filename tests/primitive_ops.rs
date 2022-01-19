use assert_cmd::Command;

const NEGATIVE_OUTPUT: &str = "-10865200\n\
4325235\n\
-66\n\
-54526\n\
-200\n\
-200\n\
-199\n\
54320\n\
-194\n\
-54514\n\
199\n\
-6400\n\
-7\n\
134217721\n\
200\n\
56\n\
-200\n\
Ｘ\n\
-200\n\
-200.0\n\
-200.0\n\
-355007752042\n\
-2209332\n\
-2178255\n\
-6589093\n\
-15647\n\
-6534767\n\
-6534766\n\
16400\n\
-6496841\n\
-6513241\n\
6534766\n\
-209112544\n\
-204212\n\
576460752303219276\n\
6534767\n\
-111\n\
18833\n\
䦑\n\
-6534767\n\
-6534767.0\n\
-6534767.0\n\
-1274270.6\n\
4325411.5\n\
-7.8186665\n\
-54349.457\n\
-23.456\n\
-23.456\n\
-22.456\n\
23.456\n\
-23\n\
-23\n\
￩\n\
-23\n\
-23\n\
-23.45599937438965\n\
-6706913.519214\n\
4325311.543211\n\
-41.152263\n\
-54449.456789\n\
-123.456789\n\
-123.456789\n\
-122.456789\n\
123.456789\n\
-123\n\
-123\n\
ﾅ\n\
-123\n\
-123\n\
-123.45679\n";

#[test]
fn negative_values() {
    let mut command = Command::cargo_bin("robusta").unwrap();

    command
        .env("ROBUSTA_CLASSPATH", "robusta-java-runtime/target/robusta-java-runtime.jar:robusta-java-test/target/robusta-java-test.jar")
        .args("PrimitiveOps -200 -6534767 -23.456 -123.456789".split_whitespace())
        .assert()
        .success()
        .code(0)
        .stdout(NEGATIVE_OUTPUT)
        .stderr("");
}
