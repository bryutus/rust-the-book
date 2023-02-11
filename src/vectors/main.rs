fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100]; // program to panic
    let does_not_exist = v.get(100);
    println!("{}", does_not_exist.is_none());

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("The first element is: {}", first);

    // Iterating over the Values in a Vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    println!("{:?}", v);

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for c in row {
        println!("{:?}", c);
    }
}
