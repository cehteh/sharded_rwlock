= ShardedRwLock, aka 'not so bad global rwlock'

Will become in sync with the 'sharded_mutex' crate on demand. (send PR or contact me)

<!---
This library provides global locks for (pseudo-) atomic access to data without memory overhead
per object. Concurrency is improved by selecting a Mutex from a pool based on the Address of
the object to be locked.

Even being sharded, these Mutexes act still as global and non-recursive locks. One must not
try to lock another object while a lock is already hold, otherwise deadlocks are around the
corner.
--->
