#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Ipv4Addr {

}

#[derive(Debug)]
struct Ipv6Addr {

}

// Can put any type of data inside an enum
#[derive(Debug)]
enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do some stuff here
    }
}

fn main() {

    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    println!("four: {:#?}", four);

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from(":1"),
    };

    println!("home: {:#?}", home);
    println!("loopback: {:#?}", loopback);

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("home2: {:#?}", home2);
    println!("loopback2: {:#?}", loopback2);

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    println!("home3: {:#?}", home3);
    println!("loopback3: {:#?}", loopback3);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this will produce an error
    // let sum = x + y;
}


fn route(ip_kind: IpAddrKind) {}