
fn main() {
    let v: Vec<i64> = Vec::new();

    let u: Vec<f64> = vec![1.0, 2.0, 3.75];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![11, 219, 3913, 41, 5123];

    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let fourth: Option<&i32> = v.get(3);

    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element"),
    }

    // this will give an error 
    // let does_not_exist = &v[99];

    // this will give an Option
    let does_not_exist = v.get(100);

    if let Some(something) = does_not_exist {
        println!("The 101st element is {something}");
    } else {
        println!("None value index is too big");
    }
}
