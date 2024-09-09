fn main() {
    // Struct
    struct Book {
        title: String,
        author: String,
        price: String,
        pages: u32,
        available: bool,
    }

    #[derive(Debug)]
    struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User{
        active: true,
        username: "habib".to_string(),
        email: "habib@telesom.com".to_string(),
        sign_in_count: 1,
    };

    // Return a struct from function
    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    };


    // Create instances from other instances
    let user2 = User{
        email: String::from("another@gmail.com"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-Like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
