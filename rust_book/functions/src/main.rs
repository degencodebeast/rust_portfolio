fn main() {
    println!("Hello, world!");

    another_function();


    let x: i32 = plus_one(8);
    
}


fn another_function() {
    println!("Are ordinals something sustainable ?");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}