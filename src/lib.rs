use std::sync::{Condvar, Mutex};

#[derive(Clone)]
pub struct WaitGroup {
    cvar: Condvar,
    count: Mutex<usize>
}

impl WaitGroup {
    pub fn new() -> WaitGroup {
        WaitGroup {
            cvar: Condvar::new(),
            count: Mutex::new(0)
        }
    }

    pub fn add(&self, n: usize) {
        let mut count = self.count.lock().unwrap();
        *count += n;
    }

    pub fn done(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
    }

    pub fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        while *count > 0 {
            count = self.cvar.wait(count).unwrap()
        }
    }
}

#[test]
fn it_works() {
}
