use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

// error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    // // explicit match
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // // propagation with ? operator (since the return type implements `FromResidual`)
    // // if File::open returns an Ok, the code will continue as normal
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // // if read_to_string returns an Err, the function will return it as io::Error.
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // // alternative way of writing the same function
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // or, without being as instructive, simply
    fs::read_to_string("hello.txt")
}

// the ? operator can be used with any return type that implements `FromResidual`, such as Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // panic macro
    // panic!("crash and burn");

    // code compiles but panics
    // let v = vec![1,2,3];
    // v[4];

    let greeting_file_result = File::open("hello.txt");

    // // when this code panics, it does not matter what error it was.
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    // // handling the case where the only error is the file not existing, panics otherwise
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => {
    //         println!("opened file {:?}", file);
    //         file
    //     },
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => {
    //                 println!("opened created file {:?}", fc);
    //                 fc
    //             },
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // // cleaner with unwrap_or_else
    // let _greeting_file = greeting_file_result.unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // // without a "hello.txt" file, this will panic
    // let _greeting_file = File::open("hello.txt").unwrap();

    // using expect method to customize the panic error message
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
