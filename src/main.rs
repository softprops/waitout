extern crate waitgroup;
use std::sync::Arc;
use std::thread;
fn main() {
    let wg = Arc::new(waitgroup::WaitGroup::new());
    for i in 0..100 {
        wg.add(1);
        let wg2 = wg.clone();
        thread::spawn(move|| {
            thread::sleep_ms(2000);
            wg2.done();
        });
    }
    wg.wait();
    println!("done")
}