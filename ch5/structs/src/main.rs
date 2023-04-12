struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // To get a specific value from a struct, we use dot notation.
    println!("user1 username is: {}", user1.username);
    println!("user1 active is: {}", user1.active);
    println!("user1 email is: {}", user1.email);
    println!("user1 sign in count is: {}", user1.sign_in_count);
}
