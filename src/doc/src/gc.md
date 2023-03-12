# Garbage Collection

## Safe Points

I've noticed that the vast majority of the time that a thread spends can
be considered a *safe point*.  The only sections that a thread is not within
a safe point will be a section that changes the set of roots of the thread,
or reads/writes from heap memory.

The set of roots of a thread are:

- the references in the operand stack
- the references in the local variable array
- the thread instance

Instructions like `iadd`, `sipush` and `if_icmpge` can globally be considered
safe points, and instructions like `invokestatic` are only unsafe when we're
moving values from the operand stack to the next frame, and in unsafe sections
of class loading and initialization.

When a thread is waiting on another thread, waiting on IO, or a synchronization
point in the program, we should consider all of these to be safe points that
won't stop GC.

I think therefore, that rather than register safe points, and solve problems
like how do you poll a waiting thread for the safe point, threads should be
registering the opposite, unsafe regions, why should a thread that is doing
1000s of pure computation instructions be stopped by a stop the world GC?

### Design

The word unsafe is overloaded by Rust already, *this is a bigger problem
we have, terms like thread & ref are overloaded by the language and we need
to think about how we refer to these*.

I'll use the term *critical region* to refer to a section in a thread that
cannot be performed during GC.

A thread should have the API


```rust
pub fn instruction(thread: &mut Thread) {
    let frame = thread.curr_frame();
    
    // Do work that isn't GC sensitive
    let int = frame.pop_int();
    let double = frame.pop_double();
    let index = frame.read_u16();
    
    let result = perform_work(int, double);
    ...

    {
        // We need to acquire some form of lock, that blocks
        // if we have to wait for GC to finish.
        let _guard = thread.enter_critical_region();
        
        let reference = frame.pop_ref();
        let object = thread.runtime.load_object(reference);
        ...
    }
    
    // If we have more work that can be performed without the
    // critical lock, there is no need to hold on to it.
    let reference = frame.load_reference(0);
    frame.push_reference(reference);
}
```

A method like `Thread::enter_critical_region` should encapsulate
everything that handles the safe/unsafe boundary:

- A thread invoking the method will return if GC is not happening.
- A thread invoking the method will block if GC is occuring, and only
    return after it has finished.
- Once a thread has got the guard, GC that is attempting to start must
    block, until the guard has been returned.

A method like `GarbageCollector::enter_safe_region` could perform the
opposite side of this relationship.

The simplest way to solve this problem seems to be having a `Mutex<()>`
in every thread, which the thread, or the GC can acquire.  This is probably
not a good implementation for performance, but we'll start here.