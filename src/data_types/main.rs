use std::io;

fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference is {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    println!("quotient is {}, floored is {}", quotient, floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is {}", remainder);

    // bool
    let t = true;
    let f: bool = false;
    println!("boolean are {} and {}", t, f);

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("char is {} and {}, {}", c, z, heart_eyed_cat);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let t: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = t.0;
    let six_point_four = t.1;
    let one = t.2;

    println!(
        "The first value: {}, second value:  {}, third value: {}",
        five_hundred, six_point_four, one
    );

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // let a = [3; 5];
    // let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
    println!("array first element: {}, second element: {}", first, second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index,
        element
    );
}
