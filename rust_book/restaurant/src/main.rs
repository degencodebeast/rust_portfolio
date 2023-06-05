use std::collections::HashMap;
use std::{fmt, io, fmt::Result, fmt::Error, cmp::Ordering};
use std::io::{Result as IoResult, Error as IoError};
use rand::Rng;

fn main() {
    let mut map = HashMap::new();

    map.insert(1,2);

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("secret_number is {}", secret_number);

}

fn function1(f: &mut fmt::Formatter<'_>) -> Result {
    write!(f, "hello")
}

fn function2() -> IoResult<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}