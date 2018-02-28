//built-in types all implement the Display format
//Our struct Rectangle does not
//
//In order to print it, then, we reuqire the use
//of the Debug trait using the ':?' specifier
//
//To use this trait, though, we must derive the
//Debug trait as follows:

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width:  u32,
}

impl Rectangle { //implementation block to specify definition only for Rectangle
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { //This is an associated function as it doesn't take self argument
        Rectangle {width: size, height: size} //Associated funcs often used for constructors
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}


//Methods are like functions, but they
//are implemented for specific structs and they
//always call self as their parameter.

fn main() {
    let rect1 = Rectangle {height:21, width: 22};

    println!("Rectangle rect1 is the rectangle that has {:?}", rect1);
    println!("\n\nOr, if your prefence is list form\nrect1 is {:#?}\n", rect1);
    //println!("The area of the rectangle is: {}", area(&rect1));
    println!(
        "The rectangle is {} square pixels\n", rect1.area()
        );

    let rect2 = Rectangle {height: 12, width: 15};
    let rect3 = Rectangle {height: 13, width: 25};

    println!("Rectangle rect2 is {:?}\n", rect2);
    println!("Rectangle rect3 is {:?}\n", rect3);

    println!("rect1 can hold rect2: {}",rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}",rect2.can_hold(&rect1));
    println!("rect1 can hold rect3: {}",rect1.can_hold(&rect3));
    println!("rect3 can hold rect1: {}",rect3.can_hold(&rect1));
    println!("rect2 can hold rect3: {}",rect2.can_hold(&rect3));
    println!("rect3 can hold rect2: {}\n",rect3.can_hold(&rect2));

    let sqr1 = Rectangle::square(4); //Associated function are called using the '::' operator
    println!("Sqr1 is {:?}",sqr1);
    println!("Is Rectangle sqr1 a square: {}", sqr1.is_square());
}

/*
 *Becuase this function is specific to Rectangles, 
 *we will implement it as a method instead
 *
 * fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
*/
