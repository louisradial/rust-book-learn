fn main() {
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("the first word is: {}", word);
    let my_string = String::from("hello world!");
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);
    let word = first_word(&my_string);
    println!("{}", word);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);
    let word = first_word(&my_string_literal);
    println!("{}", word);
    let word = first_word(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn find_first_word_wrong() {
    let mut s = String::from("hello world");
    let word = first_word_index(&s);
    println!("{word}");
    s.clear();
    // now the value in word is meaningless
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
