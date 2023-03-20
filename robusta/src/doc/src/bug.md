# Bugs

## OnceMap.get_or_init()

We appear to intermittently block on calls to `OnceMap.get_or_init()`,
which block in the underlying

```rust
fn ensure_value(&self, key: &K) {
        self.map.upsert(key.clone(),
                        || Once::new(),
                        |_| {},
        )
    }
```

This must mean that the underlying lock in the `CHashMap` is blocked at this point,
if we think about this, there are two major possible causes here:

- the lock on the map itself
- the lock on a bucket within the map

The intermittent issue, occuring at different locations across different runs
is interesting when running a single threaded application.  This definitely
requires looking at our data structure for `OnceMap` and how this could be
improved before we begin to attempt solving other problems.

