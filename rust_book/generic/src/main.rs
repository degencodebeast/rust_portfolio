struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> & T {
        &self.x
    }
}


impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

use generic::{Summary, Tweet, NewsArticle};

fn main() {
    println!("Hello, world!");

    let number_list = vec![32, 50, 12, 50, 100];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'a', 'b', 'c'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    let tweet = Tweet {
        username: String::from("elon"),
        content: String::from("buy doge pls"),
        reply: false,
        retweet: false,
    };
}

 
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news ! {}", item.summarize());
}

fn notify3(item: &(impl Summary + Display)) {

}

// weird that they chose + for saying that the type needs to implement both traits
fn notify4<T: Summary + Display>(item: &T) {

}



fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

fn some_function2<T, U>(t: &T, u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{
    19
}


fn returns_summarizable(switch: bool) -> impl Summary {
    if !switch {
        Tweet {
        username: String::from("vitalik"),
        content: String::from("Don't overload ethereum social consensus layer pls"),
        reply: false,
        retweet: false,
        }
    } else {
        NewsArticle {
            headline: String::from("Gary Gensler has undisclosed holdings of btc"),
            location: String::from("New York Empire State of Mind"),
            author: String::from("SBF"),
            content: String::from("I'm kidding, this isn't defamation it's satire")
        }
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}


impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}




impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

















