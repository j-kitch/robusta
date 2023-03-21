use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{RwLock, RwLockReadGuard};

use chashmap::CHashMap;
use nohash_hasher::BuildNoHashHasher;

use crate::java::Reference;

/// A value of [`Once<T>`] will have it's internal value initialized once, only one call to
/// initialize the value will succeed, and all other calls will wait for that value to be set
/// before returning.
///
/// There is a common pattern in the JVM to require values that are only computed once, using
/// potentially slow expensive operations, that are readable and awaitable in a thread safe
/// manor.
pub struct Once<T> {
    value: RwLock<Option<T>>,
}

impl<T> Once<T> {
    pub fn new() -> Self {
        Once {
            value: RwLock::new(None),
        }
    }

    /// Get the value in the once, or if nobody has began to set the value,
    /// set the value using the supplied [`F`].
    pub fn get_or_init<F>(&self, f: F) -> &T
        where F: FnOnce() -> T
    {
        let mut value = self.value.write().unwrap();
        let ptr = value.get_or_insert_with(f) as *mut T;
        unsafe { ptr.as_ref().unwrap() }
    }

    pub fn current(&self) -> RwLockReadGuard<Option<T>> {
        self.value.read().unwrap()
    }
}

/// A once group is a collection of once initialized values, indexed by keys and
/// providing a guarantee that all values in the group are pinned once initialized.
///
/// If you have one value to synchronize, [`Once<T>`] can be used, but if you have a collection
/// of values to have their initialization synchronized, based on a given key,
/// then [`OnceMap<K, V>`] provides the correct abstraction.
pub struct OnceMap<K: Eq + Hash + Clone, V> {
    map: CHashMap<K, Once<Box<V>>>,
}

impl<K: Eq + Hash + Clone, V> OnceMap<K, V> {
    pub fn new() -> Self {
        OnceMap { map: CHashMap::new() }
    }

    pub fn get_or_init<F>(&self, key: K, f: F) -> &V
        where F: FnOnce(&K) -> V
    {
        self.ensure_value(&key);
        let once = self.map.get(&key).unwrap();
        let value = once.get_or_init(|| Box::new(f(&key)));
        let pointer = value.as_ref() as *const V;
        unsafe { pointer.as_ref().unwrap() }
    }

    fn ensure_value(&self, key: &K) {
        self.map.upsert(key.clone(),
                        || Once::new(),
                        |_| {},
        )
    }
}

impl OnceMap<String, Reference> {
    pub fn current_values(&self) -> HashSet<u32, BuildNoHashHasher<u32>> {
        // TODO: Only way I can see of getting all the entries?
        let references: Vec<u32> = vec![0; self.map.len()];
        let idx: usize = 0;

        self.map.retain(|_, value| {
            let reference = value.current();
            let reference = reference.as_ref().unwrap().clone();
            let reference = reference.as_ref().0;

            // Not happy with this, might need to implement our own CHashMap?
            unsafe {
                let ptr = references.as_ptr().add(idx).cast_mut();
                ptr.write(reference);
                let idx_ptr = &idx as *const usize;
                let idx_ptr = idx_ptr.cast_mut();
                idx_ptr.write(idx_ptr.read() + 1);
            };

            true
        });

        references.iter()
            .map(|r| *r)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn once() {
        let once = Once::new();

        thread::scope(|s| {
            s.spawn(|| once.get_or_init(|| 10));
            s.spawn(|| once.get_or_init(|| 20));
            s.spawn(|| once.get_or_init(|| 30));
        });

        let value = once.get_or_init(|| 40);
        assert!(*value == 10 || *value == 20 || *value == 30);
    }

    #[test]
    fn once_map() {
        let map: OnceMap<i32, i32> = OnceMap::new();

        thread::scope(|s| {
            s.spawn(|| map.get_or_init(10, |x| *x));
            s.spawn(|| map.get_or_init(10, |x| x * 2));
            s.spawn(|| map.get_or_init(10, |x| x * 3));
        });

        let value = map.get_or_init(10, |x| x * 4);
        assert!(*value == 10 || *value == 20 || *value == 30);
    }
}