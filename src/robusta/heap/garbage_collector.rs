use std::ptr::slice_from_raw_parts_mut;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::heap::allocator::HEAP_SIZE;

use crate::instruction::stack::sipush;

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
    source: Source,
}

impl CopyGeneration {
    pub fn new() -> Self {
        CopyGeneration {
            blue: Data::new(),
            green: Data::new(),
            source: Source::Blue,
        }
    }

    pub fn used(&self) -> usize {
        self.source_dest().0.used.load(Ordering::SeqCst)
    }

    fn source_dest(&self) -> (&Data, &Data) {
        match self.source {
            Source::Blue => (&self.blue, &self.green),
            _ => (&self.green, &self.blue)
        }
    }

    pub fn allocate(&self, size: usize) -> *mut u8 {
        let (source, _) = self.source_dest();
        source.allocate(size).cast_mut()
    }

    /// Copy the data at &data[start..(start+size)] from the live source set to the new set,
    /// and return the new start address for the object.
    pub fn copy(&self, start: usize, size: usize) -> *mut u8 {
        let (source, dest) = self.source_dest();
        let source = &source.raw[start..start + size];
        let dest_ptr = dest.allocate(size).cast_mut();

        let mut dest = unsafe {
            slice_from_raw_parts_mut(dest_ptr, size).as_mut().unwrap()
        };

        dest.copy_from_slice(source);
        dest_ptr
    }
}

enum Source {
    Blue,
    Green,
}