struct User {
    active: bool,
    username: &str, // require lifetimes usage
    email: &str,    // require lifetimes usage
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "email@example.com",
        sign_in_count: 1,
    };
}
