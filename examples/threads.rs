extern crate waitout;

use waitout::WaitGroup;
use std::sync::Arc;
use std::thread;

fn main() {
    let wg = Arc::new(
        WaitGroup::new(0)
    );
    for _ in 0..100 {
        wg.add(1);
        let wg2 = wg.clone();
        thread::spawn(move || {
            thread::sleep_ms(2000);
            wg2.done();
        });
    }
    wg.wait();
    println!("done")
}
