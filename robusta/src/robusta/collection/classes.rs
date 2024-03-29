use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

use parking_lot::RwLock;

use crate::method_area::ObjectClass;

pub struct Classes {
    loading: RwLock<HashMap<String, ClassLoad>>,
    initialized: RwLock<HashMap<String, ClassLoad>>,
    classes: RwLock<HashMap<String, Value>>,
}

impl Classes {
    pub fn new() -> Self {
        Classes {
            loading: RwLock::new(HashMap::new()),
            initialized: RwLock::new(HashMap::new()),
            classes: RwLock::new(HashMap::new()),
        }
    }

    pub fn load_class<F>(&self, name: &str, load_class: F) -> ClassRef
        where F: FnOnce(&str) -> ObjectClass
    {
        let (creator, waiter) = self.find_status(name);
        if let Some(creator) = creator {
            // no other thread will read the status of the class, so we can insert it into the
            // data structures that we want here!
            let class = load_class(name);
            let mut classes = self.classes.write();
            classes.insert(name.to_string(), class.into());
            creator.done();
        }

        // At this point, we simply need to wait for the status to be good.
        waiter.wait();
        let classes = self.classes.read();
        let class = classes.get(name).unwrap();
        class.borrow()
    }

    pub fn initialize<F>(&self, name: &str, initialize: F)
        where F: FnOnce(&str)
    {
        let (creator, waiter) = self.find_init(name);
        if let Some(creator) = creator {
            // no other thread will read the status of the class, so we can insert it into the
            // data structures that we want here!
            initialize(name);
            creator.done();
        }

        // At this point, we simply need to wait for the status to be good.
        waiter.wait();
    }

    /// If the class is loaded, return a loading status that tells us it is ready!
    /// Else if nobody else has started, give us that indicator, else give us a blocking
    /// wait upon it being finished!
    fn find_status(&self, name: &str) -> (Option<ClassCreator>, ClassWaiter) {
        let mut loading = self.loading.write();
        if !loading.contains_key(name) {
            let load = ClassLoad::new();
            let creator = load.creator();
            let waiter = load.waiter();
            loading.insert(name.to_string(), load);
            (Some(creator), waiter)
        } else {
            let load = loading.get(name).unwrap();
            (None, load.waiter())
        }
    }

    fn find_init(&self, name: &str) -> (Option<ClassCreator>, ClassWaiter) {
        let mut initialized = self.initialized.write();
        if !initialized.contains_key(name) {
            let load = ClassLoad::new();
            let creator = load.creator();
            let waiter = load.waiter();
            initialized.insert(name.to_string(), load);
            (Some(creator), waiter)
        } else {
            let load = initialized.get(name).unwrap();
            (None, load.waiter())
        }
    }

}

struct Value {
    class: Arc<ObjectClass>
}

impl Value {
    pub fn borrow(&self) -> ClassRef {
        ClassRef {
            class: self.class.as_ref() as *const ObjectClass
        }
    }
}

impl From<ObjectClass> for Value {
    fn from(value: ObjectClass) -> Self {
        Value { class: Arc::new(value) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ClassRef {
    class: *const ObjectClass
}

impl ClassRef {
    pub fn new(obj_class: *const ObjectClass) -> Self {
        ClassRef {
            class: obj_class
        }
    }
}

impl Deref for ClassRef {
    type Target = ObjectClass;

    fn deref(&self) -> &Self::Target {
        unsafe { self.class.as_ref().unwrap() }
    }
}

struct ClassLoad {
    inner: Arc<RwLock<InnerLoad>>
}

struct InnerLoad {
    done: bool,
    waiters: Vec<SyncSender<()>>
}

impl ClassLoad {
    pub fn new() -> Self {
        ClassLoad {
            inner: Arc::new(RwLock::new(InnerLoad {
                done: false,
                waiters: vec![],
            }))
        }
    }
    pub fn creator(&self) -> ClassCreator {
        ClassCreator {
            loader: self.inner.clone(),
        }
    }

    pub fn waiter(&self) -> ClassWaiter {
        let mut inner = self.inner.write();
        if inner.done {
            ClassWaiter { recv: None }
        } else {
            let (send, recv) = sync_channel(1);
            inner.waiters.push(send);
            drop(inner);
            ClassWaiter { recv: Some(recv) }
        }
    }
}

struct ClassCreator {
    loader: Arc<RwLock<InnerLoad>>
}

impl ClassCreator {
    pub fn done(&self) {
        let mut inner = self.loader.write();
        inner.done = true;
        for sender in &inner.waiters {
            sender.send(()).unwrap();
        }
        inner.waiters.clear();
    }
}

struct ClassWaiter {
    recv: Option<Receiver<()>>,
}

impl ClassWaiter {
    pub fn wait(&self) {
        if let Some(recv) = &self.recv {
            recv.recv().unwrap()
        }
    }
}


