use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

pub struct AppendOnlyMap<K: Eq + Hash + Clone, V: Clone> {
    map: RwLock<HashMap<K, Mutex<State<V>>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> AppendOnlyMap<K, V> {
    pub fn new() -> Arc<Self> { Arc::new(AppendOnlyMap { map: RwLock::new(HashMap::new()) }) }

    pub fn get_or_insert<F>(self: Arc<Self>, key: K, factory: F) -> Receiver<V> where F: FnOnce() -> V {
        let (send, recv) = sync_channel(1);

        {
            let mut map = self.map.write().unwrap();
            if !map.contains_key(&key) {
                map.insert(key.clone(), Mutex::new(State { value: None, pending_waits: Vec::new() }));
                drop(map);

                let value = factory();
                let map = self.map.read().unwrap();
                let mut state = map.get(&key).unwrap().lock().unwrap();
                state.value = Some(value.clone());
                for wait in state.pending_waits.iter() {
                    wait.send(value.clone()).unwrap();
                }
                state.pending_waits.clear();
            }
        }

        let map = self.map.read().unwrap();
        let mut state = map.get(&key).unwrap().lock().unwrap();
        if state.value.is_some() {
            send.send(state.value.clone().unwrap()).unwrap();
        } else {
            state.pending_waits.push(send);
        }

        recv
    }
}

struct State<V> {
    value: Option<V>,
    pending_waits: Vec<SyncSender<V>>,
}