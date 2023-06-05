fn main() {
    let r: &'a i32; 

    {
        let x: 'a = 5;
        r = &x;
    }
    println!("r: {}", r);
}
