fn main() {
    let s1 : String = gives_ownership();
    let s2 : String = String::from("hello");
    let s3 : String = takes_and_gives_back(s2);
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {},", s5, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// Ownership and Functions

fn ownership_and_functions() {
    let s = String::from("hello");
    println!("{s}, world!");
    takes_ownership(s);// s's value moves into the function
    // println!("{s}"); // s is no longer valid
    let x = 5;
    makes_copy(x); // i32 is Copy, so x is still valid
    println!("{x}");
} // as s's value as moved, nothing happens on drop

fn makes_copy(x: i32) {
    println!("makes copy of some i32");
}

fn takes_ownership(s: String) {
    println!("{s}");
} // drop is called, freeing the memory

// Variables and Data Interacting with Move/Clone

fn move_and_copy() {
    let x = 5;
    let y = x;
    println!("{y}");
    println!("{x}");
    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// The String Type

fn append_to_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}
