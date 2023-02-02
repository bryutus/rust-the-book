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

    let my_string = String::from("hello, world");

    // Stringのスライスを引数として渡す
    let word = first_word(&my_string[..]);
    println!("the first word is {}", word);

    // 文字列リテラルのスライスを引数として渡す
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("the first word is {}", word);

    // 文字列リテラルは「それ自体がすでに文字列スライスなので」、スライス記法なしでも機能する
    let word = first_word(my_string_literal);
    println!("the first word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
