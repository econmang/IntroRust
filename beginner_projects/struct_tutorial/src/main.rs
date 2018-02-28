struct User {
    username: String, //the named areas of the struct are called fields
    email: String,
    sign_in_count: u64,
    active: bool,
}

/*Further, note that Rust expects lifetime identifiers
 * so we cannot add fields that are references without
 * some changes
 *
 * ex.)
 * struct User {
 *     username: &str,
 *     email: &str,
 *     sign_in_count: u64,
 *     active: bool,
 * }
 *
 * will result in error due to &str types
 *
 *
 * */

//Constructor to define User instance
fn build_user(email: String, username: String) -> User {
   User {
       username: username,
       email: email,
       sign_in_count: 1,
       active: true,
   }
}

//Constructor using field init shorthand to shorten process
fn short_build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

//Can also create tuple structs with no named specifiers
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Note: you can also create structs with no fields
//These are known as Unit Structs and are normally used to
//implement traits

fn main() {
 //Create an instance of User
 //Note: you do not have to specify fields in same order created
 let user1 = User {
     email: String::from("potatoes@gmail.com"),
     username: String::from("potatomang"),
     active: true,
     sign_in_count: 1,
 };

 //A mutable instance of the same struct
 //Every field must be mutable in this instance
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
 };

    //you can use dot notation to access certain fields
    user2.email = String::from("kagebunshinnojutsu@gmail.com");

    let user1 = build_user(String::from("shweet@yahoo.com"),String::from("potatoface"));
    let user2 = build_user(user1.email, user1.username);

    //A similar assignment for user2 uses struct build syntax
    //This uses '..' to state that all unspecified areas following
    //are the same for as the other instance.

    let user2 = User {
        username: String::from("shweet"),
        email: String::from("Hola@yahoo.com"),
        ..user1
    };

    let black = (0,0,0);
    let origin = (0,0,0);
    
    //can index tuple structs like so:
    println!("The integer input for black is x: {}, y: {}, z: {}.", black.0,black.1,black.2);


}
