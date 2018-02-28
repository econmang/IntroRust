fn main() {
    let iterations = 4;
    let mut count = 0;
    while count < iterations {
        println!("Hello, world!");
        count = count + 1;
    }


    println!();
    println!("Now to use a while to work with iterable array:");
    let a = [10,20,30,50,120,22];
    count = 0;
    while count < 6 {
        println!("{}",a[count]);
        count = count + 1;
    }

    println!();
    println!("The above example is slow, though, as it do runtime check to make sure index not out of bounds");
    println!("To speed this up, we will work with iteration of array:");

    for element in a.iter() {
        println!("{}", element);
    }

    println!();
    println!("Now to have a little fun with with a for loop:");

    for i in (1..11).rev() {
        println!("{}",i);
    }
    println!("LIFTOFF!");
}
