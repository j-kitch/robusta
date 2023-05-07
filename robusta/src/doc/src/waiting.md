# Waiting

The implementation of waiting is intimately tied to synchronization.

- The current thread must own the object's monitor.
- All sync on the object's monitor is relinquished.
- Added to the wait set for the object.
- Enter a safe region.
- Thread is then "dormant" until one of 4 conditions occurs.

1) Another thread notifies the object, and this thread is arbitrarily chosen.
2) Another thread invokes notifyAll.
3) Another thread interrupts our thread.
4) The specified amount of time has passed.

- Upon waking up from the dormant state, the thread then is removed from the
    wait set and competes for access to the monitor in the usual way.