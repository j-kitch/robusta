# Project Structure

A JVM is a large piece of software with a lot of different components,
and since it's my first time building a JVM there are a lot of unknowns
to deal with about the architecture.

My initial aim is to build out the JVM in a *breadth-first* approach,
introducing very basic versions of every major component into the project
before fully implementing any one piece.

From experience, going too deep implementing one component before I've got
a proper understanding of all the pieces that will touch it will make changes
harder.

The major components that we need to implement are:

- Command Line Interface
  - Argument & Option Parsing
  - Additional Commands
  - Configuring and running the JVM
- The common runtime areas:
  - Per Class Runtime Constant Pool
  - Method Area
  - Heap
- Per Thread Structures:
  - Thread
  - JVM Frame Stack
  - Native Method Stack
- Instruction Set Implementation
- Class Loading:
  - Bootstrap Class Loading
  - User Defined Class Loading
- Garbage Collection:
- Just In Time Compilation