# waitout

Waitout provides a simple interface for tracking and awaiting the completion of multiple
asynchounous tasks.

## usage

It's sometimes useful fan out independant tasks asynchronously for efficient completion of
an aggregate task. Asynchronous tasks may sometimes be staged in various scopes making it difficult
to maintain the current state of their completion. Some libraries have interfaces like [CountDownLatches](http://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CountDownLatch.html) and [WaitGroups](https://golang.org/pkg/sync/#WaitGroup) to help solve this problem. Absent of a similar solution in rust, the motivation for waitout was born.

Waitout is a simple wrapper around of few syncronousization primatives that enable staged task completion more straight forward.
The idea is simple, keep reference create a shared counter that increments for every task you wish to await,
when a task completes decrement that counter. Then the counter reaches 0, the current thread may proceed.

```rust
extern crate waitout;

use std::sync::Arc;
use std::thread;
use waitout::WaitGroup;

fn main() {
    let wg = Arc::new(WaitGroup::new(0));
    for _ in 0..100 {
        wg.add(1);
        let wg2 = wg.clone();
        thread::spawn(move|| {
            thread::sleep_ms(2000);
            wg2.done();
        });
    }
    wg.wait();
    println!("all set")
}
```


Doug Tangren (softprops) 2015
