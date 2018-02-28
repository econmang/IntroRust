extern crate mylib;

use mylib::Summarizable;
use mylib::Tweet;
use std::cmp::PartialOrd;
/*



rgest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
         if item > largest {
             largest = item;
        }
    }
        
       largest
    }

    fn largest_char(list: &[char]) -> char {
       let mut largest = list[0];

       for &item in list.iter() {
          if item > largest {
              largest = item;
          }
       }

       largest
    }
                                                                                        

 * */

//preceding line shows two funcs that essentially serve the same function but for different types
//
//To make this simpler, we can use generics to generalize functionality; this will make it work for
//all concrete data types by using a stand-in type

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item; 
        }
    }

    largest
}

//struct demonstration generics
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//implementations can be made for specific types, T:
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//stuct demonstrating multiple generics in one point
struct ThreeDPoint<T,U,V> {
    x: T,
    y: U,
    z: V,
}

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%",
                self.high_temp,self.low_temp,self.chance_of_precipitation)
    }
}

fn main() {

    let integer = Point {x:5, y:10};
    println!("The integer point given is: ({},{})", integer.x, integer.y);

    let float = Point {x: 1.0, y: 4.0};
    println!("The float point given is ({},{})", float.x, float.y);
    println!("Specifically, the x component is: {}", float.x());
    println!("The distance of the given float from the origin is: {}",float.distance_from_origin());

    //note: generics must be of the same type in structs so:
    //let wont_work = Point {x: 5, y: 4.0};
    //will fail

    let int_float_int = ThreeDPoint {x: 4, y: 10.0, z: 7};
    println!("The 3D Point given is ({},{},{})", int_float_int.x, int_float_int.y, int_float_int.z);


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];

    let result = largest(&char_list);
    println!("The largest cahr is {}", result);
}
