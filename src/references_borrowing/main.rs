fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    {
        let r1 = &mut s;

        println!("{}", r1);
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    let _r2 = &mut s;

    let mut c = String::from("hello");

    let t1 = &c; // 問題なし
    let t2 = &c; // 問題なし
                 // let t3 = &mut c; // 大問題!

    println!("{} and {}", t1, t2);

    // Dangling References
    let reference_to_nothing = dangle();
    println!("reference_to_nothing is '{}'", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
  // 何も起こらない

fn change(some_string: &mut String) {
    some_string.push_str(", world");

    println!("{}", some_string);
}

fn dangle() -> String {
    let s = String::from("hello");

    s // &sの場合、String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
