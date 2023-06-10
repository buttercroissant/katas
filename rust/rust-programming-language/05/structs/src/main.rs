// structs
struct User {
    action: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        sign_in_count: 1,
        active: true,
        username: String::from("someusername123"),
        email: String::from("email@example.com"),
    }
    user2.email = String::from("anotheremail@example.comm")

    let user3 = User {
        active: user1.active, 
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("user4@example.com"),
        ..user1
    }

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}