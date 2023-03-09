use std::{thread, time::Duration};

#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
}

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // // closure type inference and annotation
    // // no type annotations are needed at first
    // let example_closure = |x| x;
    // // as soon as the closure is called, the input and output types are inferred
    // let s = example_closure(String::from("hello"));
    // // the closure expects a String now, so the following code won't compile
    // // let n = example_closure(5);

    // capturing references or moving ownership
    // defining and calling a closure that captures an immutable reference
    let list = vec![1,2,3,];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before caling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    // defining and calling a closure that captures a mutable reference
    let mut list = vec![4,5,6,];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // cannot borrow as immutable because it is already borrowed as mutable by the closure
    // println!("Before caling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    // forcing the closure for the thread to take ownership
    let list = vec![8,9,10,];
    println!("Before definining closure: {:?}", list);
    // can't spawn new thread with just borrowing, as the closure may outlive the current function
    // thread::spawn(|| println!("From thread: {:?}", list)).join().unwrap();
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
    // can't borrow a moved value
    // println!("After spawning a new thread: {:?}", list);
}
