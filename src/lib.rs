use std::sync::{Condvar, Mutex};

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
        self.cvar.notify_one();
    }

    pub fn done(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cvar.notify_one();
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
