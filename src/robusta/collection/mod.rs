use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread::spawn;

/// The `AppendMap` is a concurrent, thread safe map that doesn't allow removal, or updating
/// a value in the map.  It is intended to be *"appended"*, allowing thread safe concurrent access
/// to the values of the map as the map itself mutates.
///
/// Within the JVM there is a common use case for multi-threaded access to an append map, where
/// each thread competes to initially set a value in the map, using a potentially expensive blocking
/// operation.
///
/// This breaks the default behaviour of maps like `HashMap` where `insert` shares the same mutable
/// reference as all mutable references to the map's values.
pub struct AppendMap<K: Eq + Hash + Clone + Send + Sync + 'static, V: Clone + Send + Sync + 'static> {
    map: RwLock<HashMap<K, Mutex<State<V>>>>,
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static, V: Clone + Send + Sync + 'static> AppendMap<K, V> {
    pub fn new() -> Arc<Self> { Arc::new(AppendMap { map: RwLock::new(HashMap::new()) }) }

    pub fn get(self: &Arc<Self>, key: &K) -> Option<Arc<V>> {
        let map = self.map.read().unwrap();
        map.get(key).and_then(|state| state.lock().unwrap().value.clone())
    }

    pub fn get_or_insert<F>(self: &Arc<Self>, key: &K, factory: F) -> Arc<V> where F: FnOnce() -> V {
        // Create and set the state and value if we are the first user to attempt to set a value
        // against this node.
        let is_new_state = self.try_insert_state(key);
        if is_new_state {
            let value = factory();
            self.set_value(key, value);
        }
        self.wait_for_value(key)
    }

    /// On some occasions, we want to have full control to supply a result ourselves,
    /// this method allows the consumer to register that they will insert, and allow them
    /// to send the value later.
    pub fn begin_insert(self: &Arc<Self>, key: &K) -> Option<SyncSender<V>> {
        let is_new_state = self.try_insert_state(key);
        let key2 = key.clone();
        if is_new_state {
            let (sender, receiver) = sync_channel(1);
            let handle = self.clone();
            spawn(move || {
                let value = receiver.recv().unwrap();
                handle.set_value(&key2, value);
            });
            Some(sender)
        } else {
            None
        }
    }

    fn wait_for_value(self: &Arc<Self>, key: &K) -> Arc<V> {
        let map = self.map.read().unwrap();
        let mut state = map.get(key).unwrap().lock().unwrap();
        if state.value.is_some() {
            state.value.as_ref().unwrap().clone()
        } else {
            let (send, recv) = sync_channel(1);
            state.pending_waits.push(send);
            drop(state); // Want to ensure that we drop the mutex before we wait to receive.
            recv.recv().unwrap()
        }
    }

    /// Concurrently check for the missing key, and insert if missing.
    ///
    /// Returns whether the value was inserted
    fn try_insert_state(self: &Arc<Self>, key: &K) -> bool {
        let mut map = self.map.write().unwrap();
        if !map.contains_key(key) {
            map.insert(key.clone(), Mutex::new(State { value: None, pending_waits: vec![] }));
            return true;
        }
        return false;
    }

    /// Set the final value against a key, and notify and consume all the pending waits on the
    /// final value.
    fn set_value(self: &Arc<Self>, key: &K, value: V) {
        let value = Arc::new(value);

        let senders: Vec<SyncSender<Arc<V>>> = {
            let map = self.map.read().unwrap();
            let mut state = map.get(key).unwrap().lock().unwrap();
            state.value = Some(value.clone());
            state.pending_waits.drain(0..).collect()
        };

        for sender in senders {
            sender.send(value.clone()).unwrap();
        }
    }
}

/// The internal state against a given key, this value is used as we want all consumers of a
/// value to wait for the first attempt to insert to complete, which may be waiting on arbitrary
/// IO bound operations, like reading a class file.
struct State<V> {
    /// The final value in the map, once this is set, the value can immediately be read from this
    /// field.
    value: Option<Arc<V>>,
    /// The waiting consumers, **before** the value has been set in `value`.  Once the value has
    /// been set, each `SyncSender<V>` is called, and removed from the pending waits.
    pending_waits: Vec<SyncSender<Arc<V>>>,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    #[test]
    fn only_first_insert_works() {
        let map = AppendMap::new();

        let foo_first = map.get_or_insert(&"foo".to_string(), || 20);
        let foo_next = map.get_or_insert(&"foo".to_string(), || 30);

        let bar_first = map.get_or_insert(&"bar".to_string(), || 40);
        let bar_next = map.get_or_insert(&"bar".to_string(), || 50);

        assert_eq!(foo_first.deref(), &20);
        assert_eq!(foo_next.deref(), &20);
        assert_eq!(bar_first.deref(), &40);
        assert_eq!(bar_next.deref(), &40);

        assert_eq!(map.get(&"foo".to_string()).unwrap().deref(), &20);
        assert_eq!(map.get(&"bar".to_string()).unwrap().deref(), &40);
    }

    #[test]
    fn multi_threaded() {
        let map = AppendMap::new();
        let (send_first, recv_first) = sync_channel(1);
        let (send_next, recv_next) = sync_channel(1);

        let t1map = map.clone();
        let t2map = map.clone();
        let t1 = thread::spawn(move || t1map.clone().get_or_insert(&10, || recv_first.recv().unwrap()));
        let t2 = thread::spawn(move || t2map.clone().get_or_insert(&10, || recv_next.recv().unwrap()));

        send_next.send(20).unwrap();
        sleep(Duration::from_millis(100));
        send_first.send(10).unwrap();

        assert_eq!(t1.join().unwrap().deref(), &10);
        assert_eq!(t2.join().unwrap().deref(), &10);
        assert_eq!(map.get_or_insert(&10, || 30).deref(), &10);
    }
}