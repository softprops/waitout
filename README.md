# waitout

[![Build Status](https://travis-ci.org/softprops/waitout.svg?branch=master)](https://travis-ci.org/softprops/waitout)

Waitout provides a simple interface for tracking and awaiting the completion of multiple
asynchounous tasks.

## api docs

Find them [here](https://softprops.github.io/waitout).

## install

Add the following to you're `Cargo.toml` file

```toml
[dependencies]
waitout = "0.1"
```

## usage

It's sometimes useful to fan out independant tasks asynchronously for efficient completion of
an aggregate task. Asynchronous tasks may sometimes be staged in various scopes making it difficult
to monitor the current state of their completion. Some languages have std library interfaces like [CountDownLatches](http://docs.oracle.com/javase/8/docs/api/java/util/concurrent/CountDownLatch.html) and [WaitGroups](https://golang.org/pkg/sync/#WaitGroup) to help solve this problem. Absent of a similar interface in rust, the motivation for waitout was born.

Waitout is a simple wrapper around of few rust synchronisation primitives that make staged task completion more straight forward.
The idea is simple, keep reference create a shared counter that increments for every task you wish to wait on.
When a task completes decrement that counter. When the counter reaches 0, the current thread may proceed.

```rust
extern crate waitout;

use std::sync::Arc;
use std::thread;
use waitout::WaitGroup;

fn main() {
    let wg = Arc::new(
        WaitGroup::new(0)
    );
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
