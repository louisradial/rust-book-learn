use std::collections::HashMap;

fn main() {
    // Vectors
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    };

    let first = &v[0];
    println!("The first element is: {first}");
    v.push(6);

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    s1 = format!("{s1}{s1}");
    println!("{s1}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // s1 has been moved here
    let s = format!("{s1}-{s2}-{s3}"); // format! does not take ownership
    println!("{s}");
    println!("{s1}-{s2}-{s3}");

    // let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "こんにちは";
    let ko = &hello[0..3]; // but not [0 .. 4] as each character is 3 bytes long apparently?
    let n = &hello[3..6];
    let ni = &hello[6..9];
    let chi = &hello[9..12];
    let ha = &hello[12..15];
    println!("{ko}{n}{ni}{chi}{ha}");

    for c in hello.chars() {
        print!("{c}");
    }
    print!("\n");

    for b in hello.bytes() {
        print!("{b} ");
    }
    print!("\n");

    //Hash Maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 0);
    scores.insert(String::from("Blue"), 10); // inserting same key overwrites old value
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Yellow")).or_insert(60); // inserts kv pair only if key is not present

    let team_name = String::from("Blue");
    assert_eq!(scores.get(&team_name).copied().unwrap_or(0), 10);
    assert_eq!(scores.get("Yellow").copied().unwrap_or(0), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value have been moved here

    let text = "hello world wonderful world";
    let mut count_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = count_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", count_map);
}
