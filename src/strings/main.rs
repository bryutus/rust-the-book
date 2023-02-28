fn main() {
    let data = "initial contents";

    let s = data.to_string();
    println!("{}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let mut l = String::from("lo");
    l.push('l');
    println!("{}", l);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let ttt = tic + "-" + &tac + "-" + &toe;
    println!("{}", ttt);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let ttt = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", ttt);

    // `String` cannot be indexed by `{integer}`
    // let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("Hola").len();
    println!("{}", len);
    let len = String::from("Здравствуйте").len();
    println!("{}", len);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
