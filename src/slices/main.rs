fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // wordの中身は5になる

    println!("the first word is: {}", word);

    // String Slices
    let e = String::from("hello");
    let len = e.len();

    let slice = &e[0..2];
    println!("{}", slice);
    let slice = &e[..2];
    println!("{}", slice);

    let slice = &e[3..len];
    println!("{}", slice);
    let slice = &e[3..];
    println!("{}", slice);

    let slice = &e[0..len];
    println!("{}", slice);
    let slice = &e[..];
    println!("{}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
