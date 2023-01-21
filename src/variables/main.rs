fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The Value of x in the inner scope is {}", x);
    }

    println!("The Value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("There ara {} spaces", spaces)
}
