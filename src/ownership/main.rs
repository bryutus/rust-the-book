fn main() {
    // Ownership and Functions
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値がムーブされ...
                        // ... ここではもう有効ではない
                        // println! {"{}", s};

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても大丈夫
    println! {"{}", x};
    // } // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

    // Return Values and Scope
    let _s1 = gives_ownership(); // gives_ownershipは、戻り値をs1に
                                 // ムーブする

    let s2 = String::from("hello");

    let _s3 = takes_and_gives_back(s2);
    // } // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、何も起きない。s1もスコープを抜け、ドロップされる。
}

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn gives_ownership() -> String {
    // gives_ownershipは、戻り値を
    // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string // some_stringが返され、呼び出し元関数に
                // ムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_stringがスコープに入る。
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}
