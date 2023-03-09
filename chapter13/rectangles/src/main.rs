#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // the sort_by_key method takes a FnMut closure as argument as it must call
    // it multiple times this closure does not capture, mutate or move out
    // anything from its environment, so it meets FnMut bound requirements
    list.sort_by_key(|r| r.area());
    println!("{:#?}", list);

    // // the following closure only implements FnOnce, as it moves a value out
    // // of the environment. the closure transfers ownership of a `String`,
    // // which does not implement `Copy`, so by the time it is called a second
    // // time, the closure no longer owns `value`, so it cannot push into the
    // // vector.
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // this time, the closure does not move any value out of the environment,
    // so it meets the FnMut bound requirements.
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {num_sort_operations +=1; r.height});
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
