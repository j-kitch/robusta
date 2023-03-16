# Completed

The currently completed functionality of the JVM is as follows.

## CLI

The CLI is not implemented yet, only the first argument is taken as a class
name to attempt to run.

## Class Loading

Class files are searched for in `./classes/` or `./classes/EmptyMain.jar`, 
the existence of `EmptyMain.jar` is purely for testing jar behaviour.

**Big Issue:** The only time that a class is loaded is when a method
is called on the class, we are currently getting away without implementing
`java/lang/String` is that the runtime & native methods knows what to
expect the object to look like!

A subset of the class file structure is currently being parsed:

- The magic constant is being checked
- The version is being read, but ignored.
- We are only parsing the following constant types:
  - Utf8
  - Integer
  - Class
  - String
  - FieldRef
  - MethodRef
  - NameAndType
- Access flags are parsed but ignored.
- The class & super class are read, but inheritance is ignored.
- Interfaces are completely ignored.
- Fields are parsed, but:
  - Field access flags are ignored
  - Field names and types are used for the method area
  - Field attributes are ignored
- Methods are parsed, but:
  - Only static & native access flags are used
  - Method names and types are used for the method area
  - Method attributes are ignored, except for
  - Code is used for loading bytecode.
- Class attributes are ignored

Validation is not occuring at any stage of class loading, a class file is
read, and then the method area takes the constant pool / fields / methods
immediately for symbolic links.

Loading, Linking, Verification, Preperation...  There is no concept of any
of these stages being isolated or checked.

If a `<clinit>` method is defined, the thread that invoked loading the
class will spawn a new java thread to run the class initialization method
and wait for its result.  **Note: This is not tested.**

## Runtime Constant Pool

The runtime constant pool loads constants of type

- Integer
- String 

And symbolic references to 

- Class
- Field
- Method

These are loaded immediately after the class file has been loaded.

## MethodArea

Fields and methods are loaded immediately after the class file 
has been loaded.  

The method area keeps track of which classes are loaded and which
are initialized.

## Heap

The heap is extremely bare-bones, it is simply a:
- hashmap of u32 to reference counted objects

There is no garbage collection, the key algorithm is extremely poor and
there is no structure to the heap at all.

This is an immediate location to begin growing the project!

## Instructions

We've only implemented the instructions that we need for now, and only the parts
of those instructions that we need:

- `ldc` We've only implemented the string/int variation of this.
- `iload_<n>`
- `aload_<n>`
- `istore_<n>`
- `astore_<n>`
- `return`
- `invokestatic` We've only implemented the native method version of this.
