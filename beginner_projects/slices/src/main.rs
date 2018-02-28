fn main() {
   /*Slices also do not have ownership
    * They refer to pieces of contiguous data*/
   let mut s = String::from("Hello World");
   let _word = first_word(&s);

   s.clear(); //This can lead to memory errors. Even though this is cleared, word still
              //contains a value of 5, and, as such, if we attempted to index s to find the
              //word after clear() is called, it would compile, but we'd reach a runtime
              //error by indexing out of bounds.
              //
              //The proposed error is corrected by string slices.

   let s = String::from("Hello World");
   let hello = &s[0..5];
   let world = &s[6..11];

   let hell = &s[..4];
   let hello_world = &s[..];
   let orld = &s[7..];
   

   println!("Differnt slice examples:");
   println!("{} {} {} {} {}\n",hello_world, hell, orld, hello, world);

   /*
    * This will throw a compiler error after clear() now
   let mut s = String::from("hello world");
   let word = better_first_word(&s);
   s.clear(); // Error!
   */
// You can also slice non-String contiguous data types:
//

   let _a = [1,2,3,4,5];
   let _a_slice = &_a[3..5];


}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/*fn better_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}*/
