/*In ownership cargo crate, there is an example similar to first ex. here
 * but, it has to recall string instead of using references.*/

fn main() {
    let s1 = String::from("Hello");

    //The '&' symbol is used for references
    //Allows us to refer to some value without
    //taking ownership of it.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);


    //In order to change something being referenced, must use 'mut' keyword
    
    let mut s2 = String::from("Hello");
    println!("Before reference change: {}", s2);
    change(&mut s2);
    println!("After reference change: {}", s2);
    
    //Only one mutable reference allowed at one time
    //Ex.
    //let mut s = String::from("Hello");
    //let r1 = &mut s;
    //let r2 = &mut s;
    //This code will fail

    /*Using curly braces, you can create new temporary scope to create more than one
     * mut reference. Just not simultaneous ones*/
    //Ex.
    //let mut s = String::from("Hello");
    //{
    //let r1 = &mut s; //This creates a mutable refence in new scope
    //} //r1 now out of scope
    //
    //let r2 = &mut s;
    

    /*NOTE:
     * Cannot have mut reference if immutable reference exists,
     * though multiple immutable references are fine*/

    let s3 = no_dangle();
    println!("After using no_dangle(), s3 = '{}'", s3);
}

fn calculate_length(s: &String) -> usize { //s is a reference to a String
    s.len()
} //Here s goes out of the scope. But, since it has no ownership over String.
  //Nothing special happens.

fn change(s: &mut String) {
    s.push_str(", World");
}

/*
 * This is an example of an error of dangling reference. Rust will not allow this
fn dangle() -> &String { //states this returns ref to a String
    let s = String::from("Hello"); //s is added to scope

    &s //we return reference to s
} // s now out of scope so its memory is deleted. Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
