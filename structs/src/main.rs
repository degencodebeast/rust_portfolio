struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {

    let mut user1: User = User {
        active: true,
        username: String::from("vitalik_buterin"),
        email: String::from("vitalik@eth.com"),
        sign_in_count: 0
    };

    user1.email = String::from("vitalk.eth");

    let user2 = build_user(String::from("kh.grira@gmail.com"), String::from("Khaled Grira"));

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("founder@bgd.com"),
        sign_in_count: user1.sign_in_count
    };

    // can't call this because username of user1 has been moved to user3
    //println!("accessing user1 username: {}", user1.username);

    let user4 = User {
        email: String::from("another_one@gmail.com"),
        ..user3
    };

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0
    }
}

