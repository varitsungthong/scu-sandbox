
fn main() {

    struct User {
        active: bool,
        username : String,
        email : String, 
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("TvT123"),
        email: String::from("meow@email.com"),
        sign_in_count: 2,
    };

    user1.email = String::from("mewwwwosos@gmail.com");
    
    let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
    //all struct field are mutable by default
    //Creating a new User instance using all but one of the values from user1

    fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email,
        sign_in_count: 1,
    }
}


}


