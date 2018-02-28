extern crate minigrep_lib;

use std::env;
use minigrep_lib::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In text {}", config.filename);
    
    if let Err(e) = Config::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }   
    
    /*println!("{:?}", args);

    if args.len() == 3 {
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);

        let mut f = File::open(filename).expect("File not found.");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Something went wrong reading from the file");

        println!("With text:\n{}", contents);
    }

    else {
        println!("Not enough arguments specified.");
    }*/

}
