/*A module is a namespace that defines functions and types
and you can decide if they are public or private*/

// mod keyword denotes module
// types/defns/funcs all private by default
// pub states that mod is public

//use keyword is used to bring in modules into other files


pub mod client; //This means that client is defined in outside file
pub mod network;





#[cfg(test)]
mod tests {

    //need to bring in mods from parent module
    //use super keyword
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
