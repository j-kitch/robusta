use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use parking_lot::{RawRwLock, RwLock};
use parking_lot::lock_api::{ArcRwLockReadGuard, ArcRwLockWriteGuard};

pub struct AppendVec<V> {
    inner: RwLock<Vec<Value<V>>>,
}

impl<V> AppendVec<V> {
    pub fn get(&self, index: usize) -> Option<Ref<V>> {
        self.inner.read().get(index).map(|v| v.borrow())
    }

    pub fn get_mut(&self, index: usize) -> Option<RefMut<V>> {
        self.inner.read().get(index).map(|v| v.borrow_mut())
    }

    pub fn insert(&self, index: usize, value: V) -> RefMut<V> {
        let mut vec = self.inner.write();
        vec.insert(index, value.into());
        vec.get(index).unwrap().borrow_mut()
    }
}

impl<V: Ord> AppendVec<V> {
    pub fn get_or_insert_sorted(&self, value: V) -> (usize, RefMut<V>) {
        let value = value.into();
        let mut vec = self.inner.write();
        match vec.binary_search(&value) {
            Ok(index) => (index, vec.get(index).unwrap().borrow_mut()),
            Err(index) => {
                vec.insert(index, value);
                (index, vec.get(index).unwrap().borrow_mut())
            }
        }
    }
}

struct Value<V> {
    inner: Arc<RwLock<V>>,
}

impl<V> Value<V> {
    pub fn borrow(&self) -> Ref<V> {
        Ref { inner: self.inner.read_arc() }
    }

    pub fn borrow_mut(&self) -> RefMut<V> {
        RefMut { inner: self.inner.write_arc() }
    }
}

impl<V> From<V> for Value<V> {
    fn from(value: V) -> Self {
        Value {
            inner: Arc::new(RwLock::new(value))
        }
    }
}

impl<V: PartialEq> PartialEq for Value<V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.read().eq(&other.inner.read())
    }
}

impl<V: Eq> Eq for Value<V> {}

impl<V: PartialOrd> PartialOrd for Value<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.read().partial_cmp(&other.inner.read())
    }
}

impl<V: Ord> Ord for Value<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.read()
            .cmp(&other.inner.read())
    }
}

pub struct Ref<V> {
    inner: ArcRwLockReadGuard<RawRwLock, V>
}

impl<V> Deref for Ref<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

pub struct RefMut<V> {
    inner: ArcRwLockWriteGuard<RawRwLock, V>
}

impl<V> Deref for RefMut<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<V> DerefMut for RefMut<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}