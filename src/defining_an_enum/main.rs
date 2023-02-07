#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let quit = Message::Quit;
    println!("{:?}", quit);
    let movee = Message::Move { x: 1, y: 2 };
    println!("{:?}", movee);
    let color = Message::ChangeColor(0, 0, 0);
    println!("{:?}", color);

    let some_number = Some(5);
    println!("{}", some_number.is_some());
    let some_string = Some("a string");
    println!("{}", some_string.unwrap());
    let absent_number: Option<i32> = None;
    println!("{}", absent_number.is_none());
}
