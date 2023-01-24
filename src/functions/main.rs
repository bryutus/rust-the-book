fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeld_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    let n = plus_one(5);
    println!("The value of n is: {}", n);
}

fn another_function(x: i32) {
    println!("The value of x is {}.", x);
}

fn print_labeld_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
