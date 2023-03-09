use std::{thread, time::Duration};

fn main() {
    // println!("hi from the main thread!");
    // let handle = thread::spawn(|| {
    //     println!("hi from the spawned thread!");
    //     for i in 1..=10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    //     println!("goodbye from the spawned thread!");
    // });
    //
    // for i in 1..=5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // // when main is completed, all spawned threads are shut down, whether or not
    // // they have finished running.
    // handle.join().unwrap(); // waits for the thread to complete its task.
    //
    // println!("goodbye from the main thread!");

    let v = vec![1,2,3,];
    // the closure needs to own the captured value, so `move` must be used
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
