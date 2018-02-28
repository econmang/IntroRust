extern crate communicator;
//you can use the 'use' keyword to shorten lengthy function calls
//It does this by bringing module into the scope

pub mod a {
    pub mod series {
        pub mod of {
            fn nested_modules() { }
        }
    }
}


use a::series:of;


fn main() {
    //this call fails and produces an error if pub keyword not
    //explicitly stated in module
    communicator::client::connect();

   of::nested_modules(); //shortened from a::series::of::nested_modules() thanks to use keyword
   //note: adding 'use a::series::of::nestedmodules;' allows us to remove module names altogether
   //in function call
}
