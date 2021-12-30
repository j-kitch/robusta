# robusta

Robusta is an implementation of the Java 8
JVM in Rust, following this [specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

This project is a work in progress, as a personal project to learn about the
Rust programming language and to learn about the Java internals.

## Build

```shell
cd robusta
cargo build --release
```

## Examples 
The JVM only understands a few Java class files currently.

```shell
robusta EmptyMain

robusta PrintArgs a b c
a
b
c
 
```
