# Robusta

Robusta is an implementation of a Java Virtual Machine, written in Rust
as a personal project.

Robusta implements the [Java 8 JVM specification](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html),
Java 8 was chosen as a lot of additional features *(modules, records etc)* have been added since that I want to
avoid thinking about until I've got a working VM implementation.

## Command

The project exposes the binary `robusta`, which is intended to become a drop in
replacement for the `java` command.

### Examples

```
$ robusta EmptyMain
$
```

```
$ robusta PrintConstants
542354326
hello world
```
