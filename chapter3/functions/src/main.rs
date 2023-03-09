fn main() {
    another_function(five());
    print_labeled_measurement(five(), 'h');

    let y = {
        let x = 3;
        x + 1
    };
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    return 5;
}
