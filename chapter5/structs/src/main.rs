struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    user1.email.push(' ');
    // user1.username.clear();
    // print_user(&user1);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn print_user(usr: &User) -> () {
    println!("{} - {}", usr.username, usr.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
