/*
 *
 * You can add:
 *
 * [profile.release]
 * panic = 'abort'
 *
 * to Cargo.toml to stop panic! macro from
 * unwinding 
 *
 * */

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The above function returns any errors to the calling code; this gives it more control of
// what to do with the error
//
// This method of handling errors is called propogating

//A shortcut to achieve propogation:
//fn read_username_from_file() -> Result<String, io::Error> {
//    let mut f = File::open("hello.txt");
//    let mut s = String::new();
//    f.read_to_string(&mut s)?;
//    Ok(s)
//    
//}
//
//The ? after the function call works similarly to a match statement, and it seeks to find Ok or
//Err. It uses the Err by calling the from func of the From attribute of the error, as almost all
//error types implement this attribute.
//
//
//Also, ? can only be called after a func that returns a value of type Result


fn main() {
   // panic!("crash and burn"); this will cause program to crash with appropriate error message

   // let v = [0,4,23];
   // let a = v[100];  //This will cause an error for index out of bound (calls panic)

   /***RECOVERABLE ERRORS***/
   let _f = File::open("hello.txt");
   
   let _f = match _f {
       Ok(file) => file,
       Err(ref error) if error.kind() == ErrorKind::NotFound => {
           match File::create("hello.txt") {
               Ok(fc) => fc,
               Err(e) => {
                   panic!(
                       "Tried to create file but there was a problem: {:?}", e
                       )
               },
           }
       },
       Err(error) => {
           panic!("There was a problem opening the file: {:?}", 
                  error)
       },
   };

   let _user_name = read_username_from_file();
}


/*NOTES:
 *
 * Adding .unwrap() to expected error will call panic! macro with assigned message
 *
 * Using .expect("Message") is similar in that it calls the macro, but the "Message"
 * segment is the error we decide will go to the user
 *
 *
 *
 *
 * */
