fn main() {
    //Creating a constant in Rust
    const Y_NUM: u32 = 100_000;
    //Practicing with mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!();
    println!("Y_NUM is a constant, so its value is always: {}", Y_NUM);

    println!();
    let y = 4;
    let y = y + 2;
    let y = y * 4;

    println!("y is a shadowed variable. Its value after shadowing calls is: {}", y);
    println!();

    //Shadowing an immutable variable actually creates a new variable
    //This means that it can change type each time let is called

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces was string value \"     \" but after shadowing, it is: {}", spaces);
    //Note: This type of "re-typing" does not work with mut variables"
    
    //Destructing means to take a container type and break it into separate variables
    let tup: (u32, f64, bool) = (48, -21.22,true);
    let (a,b,c) = tup;
    if c {
        println!("The value of a is: {} and the value of b is: {}",a,b);
    }

    println!();
    //You can index tuples using a .
    let index = tup.2;
    if index {
         println!("Yay");
    }
}
