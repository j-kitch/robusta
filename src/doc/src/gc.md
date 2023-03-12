# Garbage Collection

## Safe Points

The garbage collector needs to be able to ensure that all java threads are
blocked in *safe points* before it begins the mark phase.  This is to ensure
that the set of roots is not mutating during the mark phase.

The areas that will change the set of roots of a thread are as follows:

- Pushing and popping references from the operand stack.
- Storing references to the local variables.
- Resolving a constant in a runtime const pool (even a class could invoke
    creating static fields in the VM).
- Reading or writing to the heap.
- Pushing or popping frames from the stack.
- Invoking a native method (the local variables need to be tracked).
- Interacting with the VM in a native method.

The JVM can either request the thread to be blocked in a safe point, or
request the thread to wait to enter an unsafe point.

A safe point can include waiting on a sync primitive, waiting on another
thread, waiting on blocking I/O, if we implement safe points, then we need
to be able to handle these cases.

Since almost everything that a thread does can be considered unsafe, the
simplest solution is to invoke safe points at standard locations:

- at the start or end of an instruction
- around a blocking operation

Garbage collection should occur less than 1/1,000,000 instructions, therefore
we should make entering-exiting safe points the smallest regions, to allow
the JVM to perform (rather than constantly opening/closing unsafe regions)
for all the instructions.

### Native Methods

Native methods need to maintain local variables in a way that can be
accessed by the vm.  Not just input/output variables but the references
that are used within the method.

For example,

```rust
fn native_method_that_blocks(args: &Args) -> Option<Value> {
    // All of the local variable params should be registered in the thread
    // for the VM to see.
    let params = &args.params;
    
    // We need to keep a reference to this array registered with the thread.
    //
    // If `new_array()` requires GC, then this method should block the
    // current thread in a safe region.
    let arr = args.runtime.new_array(ArrayType::Char, 20);
    
    // About to perform a blocking call, we need to register this as safe.
    let char_arr = {
        // The simplest way to track the additional refs at this point
        // is simply to pass them back to the thread!
        let _safe = args.thread.enter_safe_region();
        
        // While we wait for this block to return, this native method
        // **should not** stop GC. 
        //
        // This is why the thread needs to know about `arr` above.
        let response = io::network_call("https://example");
        
        response.to_char_array()
    }; //Dropping _safe should wait for GC to end, so we can continue.
    
    arr.as_char_array().copy_from(char_arr);
}
```

The reason that native blocking methods need to be handled this way, is that
all of our thread join/interrupt methods are native and require this implementation
to handle blocking.

- Native method locals need to be tracked like normal JVM frames.
- Native methods need to be able to explicitly enter safe regions, while
    continuing to run.
- VM calls that can block on GC need to be able to enter safe regions and
  block for native & normal methods.

How do we keep track of additional references in a native method? For now,
the simplest solution I think is to manually add them.

```rust
fn native_method(args: &Args) -> Option<Value> {
    // If new_object enters GC, we don't care yet, we have no
    // additional refs to track.
    let object = new_object();
    
    // Before we potentially enter GC with the second call to new_object,
    // register the local object ref with the thread.
    register_local(object);
    let object_two = new_object();

    {
        // Register the next local and let thread know we're blocking.
        register_local(object_two);
        let _safe = begin_blocking_region();
        
        io_call();
    }
}
```

**TODO:** This all points to needing an implementation of sync working
before we implement GC fully!

## Thread

A thread will have to register itself as entering a safe point.

A thread will have to register itself as leaving a safe point, and if
GC is occurring, it must block, and wait until it has finished. (In the
case of a normal instruction safe point check, these two are combined, but
for native methods that involve blocking, they are separate).

While a thread is within a safe point, it cannot change the set of roots
it contains.  While in a JVM frame, this is controlled, while in a native
method, this relies on the implementation being correct.

**TODO:** Could we separate the API into safe & unsafe for native methods
to guarantee this at compile time?

