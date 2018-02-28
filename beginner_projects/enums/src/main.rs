//enums allow us to quantify and limit the
//options for some object or value

/*
enum IPAddrKind {
    V4,
    V6,
}


struct IPAddr {
    kind: IPAddrKind,
    address: String,
}
*/
//This enum and struct can be represented more succinctly using one enum

enum IPAddr {
    V4(String),
    V6(String),
}

enum IPAddr_Two {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit, //No data assocaiated with it
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

/*
 * This is equivalent to the enum above with structs
struct QuitMessage;//unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32,i32,i32); //typle struct
*/

impl Message {
    fn call(&self) {
        //
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//match keyword is very similar to switch statement
//it uses pattern matching. If associated value matches
//then code across is executed
//
//note, matches in Rust are exhaustive so they must
//handle all cases from enum

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => { 
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(State) => {
            println!("State quarter from {:?}!",State );
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //etc...
}

//enumerations allow us to have variations of the same type
//So, we can define function such that they will work for
//any value of the enum (i.e. fn route(ip_add: IPAddrKind)){ }

//The possible values of an enum are called variations

fn main() {
    //let four = IPAddrKind::V4;
    //let six = IPAddrKind::V6;

   // Initialization of home/loopback as structs
   // let home = IPAddr {
   //     kind: IPAddrKind:V4,
   //     address: String::from("127.0.0.1"),
   // };

   // let loopback = IPAddr {
   //     kind IPAddrKing: V6,
   //     address: String::from("::1"),
   // };
   
   //Initialization of home/loopback as enums
   let home = IPAddr::V4(String::from("127.0.0.1"));
   let loopback = IPAddr::V6(String::from("::1"));

   //Another strength of enums is that different types can be represented by each variation
   //This is not able to be done by structs

   let home = IPAddr_Two::V4(127,0,0,1);
   let loopback = IPAddr_Two::V6(String::from("::1"));

   let m = Message::Write(String::from("hello"));
   m.call();

   //In place of null values (due to their error prone nature), Rust implement an Option enum
   //Where Option<T> can either be Some<T> or None. Some can be a value of any type and None
   //is the expression when no data is available or data is not valid

   let some_number = Some(5);
   let some_string = Some("a string");
   let absent_number: Option<i32> = None;

   let the_num = some_number.unwrap();
   println!("{}", 5 * the_num);

   /*Luckily, even through matche exhuastive, we can use pattern "placeholder" with symbol _ to
   * go through all exhaustive scope beyond a certain point*/

   let some_u8_value = 0u8;
   match some_u8_value {
       1 => println!("One"),
       3 => println!("Three"),
       5 => println!("Five"),
       7 => println!("Seven"),
       _ => (),
   }
   // The _ will match any value and the () is a unit value, so it will do nothing. Essentially any
   // u8 values other than 1,3,5,7 will do nothing.
   
   let some_u8_value = Some(0u8);
   //If we only care about one value, then matching & placeholders can be wordy
   //For these situations, we have the "if let" control flow feature.
   if let Some(3) = some_u8_value {
       println!("Three!");
   }

}
