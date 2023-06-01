
fn main() {
    let mut x = 1810;
    println!("The price of ETH is {x} USD ");

    x = 10000;
    println!("Hoping it will be {x} USD... please ..");

    let x = 5;
    let x = x + 1;
    println!("value of x is now {x}");

    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }
    println!("the value of x outside the inner scope is: {x}");


    let a = [1, 2, 3, 4, 5];
}
