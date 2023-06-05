fn main() {

    let mut s = String::from("hello");

    s.push_str(", degens");

    println!("{}", s);


    // move
    /*
    let s1: String = String::from("when is a variable no longer valid ?");
    
    let s2 = s1;
    
    println!("{}, hmm ? ", s1);
    */

    // deep copy 
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    // stack only data, they have the copy trait so no problem here
    let x = 5; 
    let y = x;
    println!("x = {}, y = {}", x, y);

    // can't give Copy trait to a type that implements the Drop in any of its parts

    let s = String::from("hello");

    takes_ownership(s);

    // this will return an error as s has been moved to takes_ownership scope 
    //println!("{}", s);

    let x: i32 = 5;
    makes_copy(x);

    // this should be fine as x has Copy trait
    println!("checking x: {}", x);


    let mut s1 = String::from("hello degens");

    let len = calculate_length(&s1);

    change(&mut s1);

    // borrowing, can't have multiple mutable references in the same scope
    // can't have immutable references and mutable reference in the same scope 
    // can have multiple immutable references in same scope, reading data is fine 
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // careful about dangling references, return the value directly or give the lifetime
    // of the return thing
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership call on: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy call on: {}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" modifying stuf huh");
}