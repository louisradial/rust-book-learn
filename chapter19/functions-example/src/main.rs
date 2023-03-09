fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    assert_eq!(
        list_of_numbers
            .iter()
            .map(|i| i.to_string())
            .collect::<String>(),
        list_of_numbers
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
    );

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    for status in list_of_statuses {
        println!("{:?}", status);
    }

    let plus_one = returns_closure();
    assert_eq!(3, plus_one(2));
}

