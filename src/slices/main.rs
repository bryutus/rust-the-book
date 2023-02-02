fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // wordの中身は5になる

    s.clear();

    println!("{}", word); // wordはまだ5を保持しているが、この値を正しく使用できる文字列は存在しない
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
