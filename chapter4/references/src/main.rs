fn main() {
    dangling_references()
}

fn dangling_references() {
    // let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
}

// fn dangle() -> &String { // returns a reference to a string
//     let s = String::from("hello"); // s is a new string
//     &s // return a reference to the string s
// } // s is dropped. its memory goes away

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// Data Races and Multiple Concurrent References

fn multiple_references() {
    let mut r = String::from("hello");
    // let r1 = &mut r;
    // let r2 = &mut r;
    // println!("{}, {}", r1, r2);
    {
        let r1 = &mut r;
        println!("{r1}");
    }
    let r2 = &mut r;
    println!("{r2}");
    let mut t = String::from("hello");
    // let t1 = &t; // no problem
    // let t2 = &t; // no problem
    // let t3 = &mut t; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", t1, t2, t3);
    let t1 = &t;
    // no problem
    let t2 = &t;
    // no problem
    println!("{} and {}", t1, t2);
    // the compiler can tell t1 and t2 are no longer in use
    let t3 = &mut t;
    // no longer a problem
    println!("{}", t3);
}

// Mutable References

fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn mutable_references()  {
//     let s = String::from("hello");
//     change(&s);
// }
//
// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

// References and Borrowing

fn borrow() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
