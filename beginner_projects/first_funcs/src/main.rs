fn main() {
    println!("Hello, world!");
    another_function();

    let y = {
        let x = 4;
        x + 6
    };

    param_function(y);

    let y = returning_function(y);
    param_function(y);

    let num_one = practice_bool(true);
    let num_two = practice_bool(false);

    println!("Should be five: {}\nShould be six: {}",num_one,num_two);
}

fn another_function() {
    println!("This is the output of another_function.");
}

fn param_function(x: i32) {
    println!("The value of the variable is: {}", x);
}

fn returning_function(num: i32) -> i32 {
    num * 2
}

fn practice_bool(condition: bool) -> i32 {
    let number = if condition {
        5
    } else {
        6
    };
    return number;
}
