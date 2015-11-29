//! waitgroup provides an interface for awaiting the completion of multiple asynchronous tasks

use std::sync::{Condvar, Mutex};

/// A waitgroup waits for a collection of tasks to complete.
/// It keeps track of tasks via shared counter
pub struct WaitGroup {
    cvar: Condvar,
    count: Mutex<usize>,
}

impl Default for WaitGroup {
    fn default() -> WaitGroup {
        WaitGroup::new(0)
    }
}

impl WaitGroup {
    /// creates a new wait group instance
    pub fn new(n: usize) -> WaitGroup {
        WaitGroup {
            cvar: Condvar::new(),
            count: Mutex::new(n),
        }
    }

    /// adds `delta` to internal counter
    pub fn add(&self, delta: usize) {
        let mut count = self.count.lock().unwrap();
        *count += delta;
        self.cvar.notify_all();
    }

    /// subtracts 1 from internal counter
    pub fn done(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cvar.notify_all();
    }

    /// blocks the current thread until wait group is complete
    pub fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        while *count > 0 {
            count = self.cvar.wait(count).unwrap();
        }
    }
}

#[test]
fn it_works() {}
