# Loading, Linking & Initializing

Based on the details in [the specification](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-5.html),
this is the process followed for loading, linking and initializing classes.

### Creation
Creation of a class `C` denoted by the name `N` consists of the construction
in the method area of a representation of that class.

Creation is triggered by another class `D`, referencing the class through
its runtime constant pool.

- A *non array class* is created by loading a binary class file of `C`,
using a class loader.
- An *array class* does not have a binary class and is not loaded by a
    class loader, they are created by the JVM directly.

Creation is the name given to the entire process.

### Loading

- The bootstrap loader attempts to parse a class file.
- If the class has a direct superclass, the symbolic reference to the 
    direct superclass is resolved (creating the superclass).
- If the class has any direct superinterfaces, the symbolic references
    to these are resolved.

### Linking

Linking is composed of verification, preparation and resolution.

The specification offers a lot of leniancy with this step, but must:

- A class is completely loaded before it is linked.
- A class is completely verified and prepared before it is initialized.

This means that resolution can be implemented lazily.

#### Verification

Verification is checking that the binary representation of the class
is structurally correct.  This is not the most important step for us right
now!

#### Preparation

Preparation is creating the static fields and zeroing them.

#### Resolution

The instructions *anewarray, checkcast, getfield, getstatic, instanceof, invokedynamic, invokeinterface, invokespecial, invokestatic, invokevirtual, ldc, ldc_w, multianewarray, new, putfield, and putstatic*
make symbolic references into the constant pool of a class, these instructions
require resolution of the symbolic reference.

### Initialization

Initialization involves invoking the `<clinit>` method of a class.

Only the instructions *new, getstatic, putstatic, or invokestatic*
can invoke initialization of a class.

A subclass init will invoke a parent init.

## Planned Approach

To keep things simple for now, it makes sense to perform this function
in the following phases.

- Loading, Verification & Preparation occur upon resolving a symbolic link to a class.
- Initialization will occur upon reaching an instruction that may invoke it.
- Resolution only occurs upon reaching an instruction that may invoke it.

This lazy approach makes sense to begin with, to do the bare minimum work required.