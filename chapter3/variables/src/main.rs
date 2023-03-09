use std::io;

fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours is equal to {THREE_HOURS_IN_SECONDS} seconds.");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");

    // let mut spaces = "    ";
    // spaces = spaces.len();
    let spaces = "    ";
    let spaces = spaces.len();
}
