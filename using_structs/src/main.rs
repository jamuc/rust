fn main() {
    println!("Hello, world!");
    // Has to be String type because otherwise email and username size would have to be known at compile time
    let user1 = create_user(String::from("john@test.com"), String::from("john"));
    println!("The users email is {}", user1.email);
    println!("The users username is {}", user1.username);
    println!("The users active state is {}", user1.active);
    println!("The users sign in count  is {}", user1.sign_in_count);

    // User 2 can just reuse certain things from user 1
    let user2 = User {
      email: String::from("another_email@gmail.com"),
      username: String::from("another_user"),
      ..user1
    };

    println!("The users email is {}", user2.email);
    println!("The users username is {}", user2.username);
    println!("The users active state is {}", user2.active);
    println!("The users sign in count  is {}", user2.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("The RBV values of black are {}, {}, {}", black.0, black.1, black.2);
    let (x, y, z) = (origin.0, origin.1, origin.2);
    println!("x: {}, y: {}, z: {} of the origin", x, y, z);
}

fn create_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 0,
  }
}

struct User {
  email: String,
  username: String,
  active: bool,
  sign_in_count: u32,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
