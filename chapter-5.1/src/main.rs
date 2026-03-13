struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;

struct Point(f32, f32, f32); // x y z
struct Colour(i16, i16, i16); // r g b
struct Quaternion(f32, f32, f32, f32); // w x y z

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("Hello user {}", user1.username);

    let mut user2 = User {
        active: true,
        username: String::from("someotherusername123"),
        email: String::from("anothersomeone@example.com"),
        sign_in_count: 1,
    };
    // struct must be mutable to change any field - cannot mark only some fields as mutable
    user2.email = String::from("yetanothersomeone@example.com");
    println!("User2 email is {}", user2.email);

    let user3 = build_user(user2.username, user1.email);
    println!(
        "Hello user {}, your email is {}",
        user3.username, user3.email
    );

    let user4 = User {
        email: String::from("another@example.com"),
        ..user1 // this moves the remaining fields from user1 to user4
                // user1.username cannot be used after this point as it has moved to use4.
                // user1.active and user1.sign_in_count can still be used as they are of types which implement the Copy trait
                // user1.email also cannot be used (as it is used to build user3), but as we are initialising user4.email with a new value we are not moving it!
    };

    println!(
        "Hello user {}, your email is {}",
        user4.username, user4.email
    );

    // tuple struct - named tuples, where name of data fields is not needed
    let col = Colour(255, 0, 0);
    let point = Point(1.0, 0.0, 1.0);

    let subject = AlwaysEqual; // unit struct - not data fields needed to instantiate
}
