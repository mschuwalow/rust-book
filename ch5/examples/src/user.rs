struct User {
    username:      String,
    email:         String,
    sign_in_count: u64,
    active:        bool,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
const BLACK: Color = Color(0, 0, 0);
const ORIGIN: Origin = Point(0, 0, 0);
fn mk_user1() -> User {
    User {
        email:         String::from("someone@example.com"),
        username:      String::from("someusername123"),
        active:        true,
        sign_in_count: 1,
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn change_username(user: User, username: String) -> User {
    User { username, ..user }
}
