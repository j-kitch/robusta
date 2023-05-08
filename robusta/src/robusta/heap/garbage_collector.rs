use std::collections::HashSet;
use std::mem::size_of;
use std::ptr::slice_from_raw_parts_mut;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{Builder, current, scope};
use std::time::Duration;

use nohash_hasher::BuildNoHashHasher;
use tracing::{debug, trace};

use crate::heap::{Heap, Heaped};
use crate::heap::allocator::{ArrayHeader, HEAP_SIZE, ObjectHeader};
use crate::java::Reference;
use crate::log;
use crate::runtime::Runtime;
use crate::thread::Thread;

struct Data {
    raw: Box<[u8]>,
    used: AtomicUsize,
}

impl Data {
    pub fn new() -> Self {
        Data {
            raw: vec![0; HEAP_SIZE].into_boxed_slice(),
            used: AtomicUsize::new(0),
        }
    }

    pub fn allocate(&self, size: usize) -> *const u8 {
        let result = self.used.fetch_update(
            Ordering::SeqCst, Ordering::SeqCst,
            |used| used.checked_add(size));

        let start_of_mem = result.expect("OOM");

        unsafe { self.raw.as_ptr().add(start_of_mem) }
    }
}

pub struct CopyGeneration {
    blue: Data,
    green: Data,
    source: AtomicBool,
    start_gc: Sender<Arc<Runtime>>,
}

unsafe impl Sync for CopyGeneration {}

impl CopyGeneration {
    pub fn new() -> Self {
        let start_gc = start_gc_thread();

        CopyGeneration {
            blue: Data::new(),
            green: Data::new(),
            source: AtomicBool::new(false),
            start_gc,
        }
    }

    pub fn used(&self) -> usize {
        self.source_dest().0.used.load(Ordering::SeqCst)
    }

    fn source_dest(&self) -> (&Data, &Data) {
        if self.source.load(Ordering::SeqCst) {
            (&self.blue, &self.green)
        } else {
            (&self.green, &self.blue)
        }
    }

    pub fn allocate(&self, runtime: Arc<Runtime>, size: usize) -> *mut u8 {
        let (source, _) = self.source_dest();


        let used = source.used.load(Ordering::SeqCst);
        let percentage = 100.0 * (used as f64) / (HEAP_SIZE as f64);
        if percentage > 25.0 {
            // we're already trying to do GC, enter safe region here!
            let current = current();
            let thread_name = current.name().unwrap();
            let all_threads = runtime.threads2.read().unwrap();
            let our_thread = all_threads.iter().find(|t| t.name.eq(thread_name)).unwrap();
            our_thread.safe.safe_region();
        }

        let allocated = source.allocate(size).cast_mut();

        // Start GC if we have used 25% of mem.
        let used = source.used.load(Ordering::SeqCst);
        let percentage = 100.0 * (used as f64) / (HEAP_SIZE as f64);

        if used > (HEAP_SIZE / 4) {
            debug!(target: log::GC, "Used {:.2}% of Gen 1 Copy Space, starting GC", percentage);
            self.start_gc.send(runtime).unwrap();
            thread::sleep(Duration::from_millis(10));
        }

        allocated
    }

    /// Copy the data at &data[start..(start+size)] from the live source set to the new set,
    /// and return the new start address for the object.
    pub fn copy(&self, start: usize, size: usize) -> *mut u8 {
        let (_, dest) = self.source_dest();
        let source = unsafe { slice_from_raw_parts_mut(start as *mut u8, size).as_mut().unwrap() };
        let dest_ptr = dest.allocate(size).cast_mut();

        let dest = unsafe {
            slice_from_raw_parts_mut(dest_ptr, size).as_mut().unwrap()
        };

        dest.copy_from_slice(source);
        dest_ptr
    }

    pub fn swap(&self) {
        let (source, _) = self.source_dest();
        source.used.store(0, Ordering::SeqCst);
        self.source.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |b| Some(!b))
            .unwrap();
    }
}

pub struct CopyCollector {
    start: Receiver<Arc<Runtime>>,
    gcs: usize,
}

pub fn start_gc_thread() -> Sender<Arc<Runtime>> {
    let (sender, receiver) = channel();

    Builder::new()
        .name("GC-Copy".to_string())
        .spawn(move || {
            let mut collector = CopyCollector::new(receiver);
            collector.run()
        }).unwrap();

    sender
}

impl CopyCollector {
    pub fn new(start: Receiver<Arc<Runtime>>) -> Self {
        CopyCollector { start, gcs: 0 }
    }

    pub fn run(&mut self) {
        loop {
            let runtime = self.start.recv().unwrap();
            self.gc(runtime);
        }
    }

    pub fn visiting(&mut self, runtime: &Arc<Runtime>, mut roots: HashSet<u32, BuildNoHashHasher<u32>>) -> HashSet<u32, BuildNoHashHasher<u32>> {
        let heap = &runtime.heap;

        let mut visited = HashSet::with_capacity_and_hasher(runtime.heap.num_objects(), BuildNoHashHasher::default());
        visited.insert(0);
        let mut remaining_to_visit = roots;

        while remaining_to_visit.len() > 0 {
            let next_object = remaining_to_visit.iter().next().unwrap().clone();
            remaining_to_visit.remove(&next_object);
            visited.insert(next_object);

            // Copy object over to new set.
            let value = heap.get(Reference(next_object));
            match value {
                Heaped::Array(mut array) => {
                    let header = unsafe { array.header.as_ref().unwrap() };
                    let start = array.header as usize;
                    let size = size_of::<ArrayHeader>() + header.length;

                    trace!(target: log::GC, gen="gen-1", obj="array", start, size, "mark & copy");

                    let new_start = heap.allocator.gen.copy(start, size);

                    let source = unsafe { slice_from_raw_parts_mut(array.header as *mut u8, size).as_mut().unwrap() };
                    let dest = unsafe { slice_from_raw_parts_mut(new_start, size).as_mut().unwrap() };
                    dest.copy_from_slice(source);

                    // Need to update pointers in heap.
                    array.header = new_start as *mut ArrayHeader;
                    array.data = unsafe { new_start.add(size_of::<ArrayHeader>()) };
                    heap.set(Reference(next_object), Heaped::Array(array));

                    // If its an array of references, we want to add all of those to the set.
                    if header.component.is_reference() {
                        remaining_to_visit.extend(array.as_ref_slice().iter()
                            .filter(|reference| !visited.contains(*reference)));
                    }
                }
                Heaped::Object(mut object) => {
                    let header = unsafe { object.header.as_ref().unwrap() };
                    let class = unsafe { header.class.as_ref().unwrap() };
                    let start = object.header as usize;
                    let size = size_of::<ObjectHeader>() + class.instance_width;

                    trace!(target: log::GC, gen="gen-1", obj="object", start, size, "mark & copy");

                    let new_start = heap.allocator.gen.copy(start, size);

                    let source = unsafe { slice_from_raw_parts_mut(object.header as *mut u8, size).as_mut().unwrap() };
                    let dest = unsafe { slice_from_raw_parts_mut(new_start, size).as_mut().unwrap() };
                    dest.copy_from_slice(source);

                    // Need to update pointers in heap.
                    object.header = new_start as *mut ObjectHeader;
                    object.data = unsafe { new_start.add(size_of::<ObjectHeader>()) };
                    heap.set(Reference(next_object), Heaped::Object(object));

                    // For every reference in the objects fields, add to set.
                    for parent in class.parents() {
                        for field in &parent.instance_fields {
                            if field.descriptor.is_reference() {
                                let reference = object.field_from(field).reference().0;
                                if !visited.contains(&reference) {
                                    remaining_to_visit.insert(reference);
                                }
                            }
                        }
                    }
                }
            }
        }

        visited
    }

    pub fn gc(&mut self, runtime: Arc<Runtime>) {
        let heap = &runtime.heap;

        let used = heap.allocator.gen.used() as f64;
        let max = HEAP_SIZE as f64;
        let perc = (100.0 * used) / max;
        if perc < 25.0 {
            debug!(target: log::GC, "Skipping GC with perc {:.2}%", perc);
            return;
        }

        debug!(target: log::GC, "Starting Gen 1 Copy Garbage Collection");

        // Ensure all threads are ready to start GC.
        let threads = runtime.threads2.read().unwrap();
        scope(|scope| {
            for thread in threads.iter() {
                Builder::new()
                    .name(format!("GC-Copy-Pause-{}", thread.name.as_str()))
                    .spawn_scoped(scope, || thread.safe.start_gc()).unwrap();
            }
        });
        debug!(target: log::GC, "All threads stopped");


        let mut roots: HashSet<u32, BuildNoHashHasher<u32>> = HashSet::with_capacity_and_hasher(runtime.heap.num_objects(), BuildNoHashHasher::default());
        for thread in threads.iter() {
            let thread_roots = thread_roots(thread.as_ref());
            roots.extend(thread_roots.iter());
        }
        roots.extend(heap_roots(runtime.heap.as_ref()).iter());

        let used = heap.allocator.gen.used();
        let percentage = (100.0 * (used as f64)) / HEAP_SIZE as f64;
        debug!(target: log::GC, gen="gen-1", used=format!("{}mb", used / 1024 / 1024), percentage=format!("{:.2}%", percentage), "Starting Mark&Copy garbage collection");

        let visited = self.visiting(&runtime, roots);

        // What is dead is dead - remove from heap.
        heap.retain(&visited);
        heap.allocator.gen.swap();

        self.gcs += 1;
        let used = heap.allocator.gen.used();
        let percentage = (100.0 * (used as f64)) / HEAP_SIZE as f64;
        debug!(target: log::GC, gen="gen-1", gc=self.gcs, used=format!("{}mb", used / 1024 / 1024), percentage=format!("{:.2}%", percentage), "Ending Mark&Copy collection");

        scope(|scope| {
            for thread in threads.iter() {
                Builder::new()
                    .name(format!("GC-Copy-Restart-{}", thread.name.as_str()))
                    .spawn_scoped(scope, || thread.safe.end_gc()).unwrap();
            }
        });
        debug!(target: log::GC, "All threads restarted");
    }
}

/// Get all the root objects that a thread has access to.
pub fn thread_roots(thread: &Thread) -> HashSet<u32, BuildNoHashHasher<u32>> {
    let mut refs = HashSet::with_hasher(BuildNoHashHasher::default());

    // The thread instance for this thread itself.
    thread.reference.map(|r| refs.insert(r.0));

    // Get all the local vars and operand stack references.
    for frame in &thread.stack {
        refs.extend(frame.local_vars.roots().iter());
        refs.extend(frame.operand_stack.roots().iter());
        refs.extend(frame.native_roots.iter());
    }

    refs
}

pub fn heap_roots(heap: &Heap) -> HashSet<u32, BuildNoHashHasher<u32>> {
    let mut refs = HashSet::with_hasher(BuildNoHashHasher::default());
    // class objects are roots
    refs.extend(heap.class_objects.current_values().iter());

    // string constants are roots
    refs.extend(heap.string_constants.current_values().iter());

    // // static fields are roots
    // refs.extend(heap.static_objects.current_values().iter());

    refs
}