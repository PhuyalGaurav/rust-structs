use std::fmt;

// Implement the Display trait for the Cordinates struct (Copilot code idk this shit)
impl fmt::Display for Cordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

fn main() {
    // Custom datatype can be mutable using mut
    let mut user1: User = User {
        id: 1,
        active: true,
        username: String::from("Gaurav"),
        email: String::from("phuyalgaurav90@gmail.com"),
        sign_in_count: 1,
        location: Cordinates(0, 0, 0),
    };

    // Changing a value
    user1.email = String::from("gauravgamer019@gmail.com");
    display_user(&user1);

    // we can use ""
    let user2: User = User {
        id: 2,
        username: String::from("manish"),
        email: String::from("manish@manish.manish"),
        ..user1
    };

    display_user(&user2);
}

struct User {
    id: u64,
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    location: Cordinates,
}

struct Cordinates(i32, i32, i32);

fn display_user(user: &User) {
    println!("User Details: {}", user.id);
    println!("Active: {}", user.active);
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign-in Count: {}", user.sign_in_count);
    println!("User Location: {}", user.location); // Fix the println statement
}
