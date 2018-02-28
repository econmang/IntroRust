/*Ownership Rules:
 * Each value in Rust has a variable that’s called its owner.
 * 
 * There can only be one owner at a time.
 * 
 * When the owner goes out of scope, the value will be dropped.
 */

//Note: Stack is for known-size space that needs to be allocated for variables and only stores a
//relativley small amount of data. Also, it is LIFO; fast and efficient

//Heap is needed when data allocation size can change/is unkownn for variables or large amount of
//data needs to be stored. Also, not very fast/efficient because it is randomly allocated/accessed
//by following pointers

fn main() {
   //the variable s is not in scope because it has not been declared 
   let _s = "hello"; //variable s brought into scope on declaration; stored on stack

   //String class is stored on heap
   //Able to be mutated and size not known at compile time
   let mut s = String::from("hello");
   s.push_str(", world!");
   println!("{}",s);



   let x = 5; 
   let _y = x;
   //Assigns value of x to y. This creates a copy of x variable and binds it to y.
   //Both vars are of known size at compile time. Shallow and deep copying are the
   //same thing for integer types. This is because their size in known at compile time,
   //so, no matter what, they are allocated to the stack.
   
   let s1 = String::from("hello");
   let _s2 = s1;
   /*This differs from x and y assignements. A string is made up of pointer to
   memory, length, and capacity. Assigning a String variable to another does
   not allocate more memory on heap. Instead, to save on space/time, the values
   of length and capacity are stored in String variable, as is the pointer.
   
   This means that rather than creating a new String content, both vars point
   to same place in memory.
   
   This means that, when drop is called after end of program, a double free 
   error exists (causing potential corruption and security vulnerabilities)
   as both vars will drop same memory location.
   
   Therefore, Rust makes first var invalid (out of scope) once second var
   points to same memory address.

   //Ex.)
   println!("{}, world", s1); //This will cause an error as s2 points to same mem address. So, s1 invalid.
   
   This invalidation of memory for s1 means that, what is a shallow copy for other languages, is a "move"
   in this context. So we say s1 moved to s2.
   */


   let s1 = String::from("hello");
   let s2 = s1.clone();

   //This is another option: option to create a deep clone. Heap data is copied at runtime, so 
   //two String vars are created with same contents, but at different memory addresses.

   println!("s1 = {}, s2 = {}", s1, s2);


   //The way Rust keeps track of this is through Copy and Drop attributes of data types. Copy lets
   //Rust know that first var is still usable due to creating new stack vars and Drop tells Rust
   //that first var is unusable after first assignment.


/* *************************************************************************************** */
// It's possible to return mult. vals using a tuple
/* *************************************************************************************** */
   let s1 = String::from("hello");

   let (s2,len) = calculate_length(s1);

   println!("The length of '{}' is {}.", s2, len);



//------------------------------------------------------------------------------------------//
   let s = String::from("hello"); // s comes into scope

   takes_ownership(s); // s's value moves into the function...
                       // ... and so is no longer valid here.

   let x = 5;          // x comes into scope
                       // but i32 is Copy, so it's okay to still use x afterward.

   makes_copy(x);

   let _s1 = gives_ownership(); // gives_ownership moves it return val into s1
   let s2 = String::from("hello"); //s2 comes into scope
   let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                      // takes_and_gives_back, which also moves its return
                                      // value into s3

//-------------------------------------------------------------------------------------------//

}

//--------------------------------------------------//
// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so
// nothing special happens. s1 goes out of scope and is dropped.
//-------------------------------------------------//

//variable s out of scope at end of program; it is popped off stack
/*At program end, x goes out of scope, then s. But, since s's value was moved, nothing
 * special happens here*/


//FUNCS USED BY MAIN()
fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{}", some_string);
}//Here, some_string goes out of scope and 'drop' is called. The backing 
 //memory is freed.

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    println!("{}", some_integer);
} //Here, some_integer goes out of scoep. Nothing special happens.

fn gives_ownership() -> String {        //gives_ownership will move its
                                        //return value into the function
                                        //that calls it.

    let some_string = String::from("hello"); //some_string comes into scope

    some_string                         // some_string is returned
                                        // and moves out to the calling
                                        // function.
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope.
    a_string // a_string is returned and moves out to the calling function.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() returns the length of a String.

    (s, length)
}
