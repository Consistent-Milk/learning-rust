struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1: User = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someemail@email.com"),
        sign_in_count: 1,
    };

    // To mutate a field of a struct instance,
    // we have to define that whole instance to be mutable
    let mut user2: User = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someemail@email.com"),
        sign_in_count: 1,
    };

    println!("Email of user1 is {}", user1.email);

    user2.email = String::from("anotheremail@email.com");

    println!("Email of user2 is {}", user2.email);

    let user3: User = User {
        active: user1.active,
        username: user1.username,
        email: user2.email,
        sign_in_count: user1.sign_in_count,
    };

    println!("Email of user3 is {}", user3.email);
}
