struct Reactangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Reactangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the reactangle is {} square pixels.",
        area(&rect)
    );
}

fn area(reactangle: &Reactangle) -> u32 {
    reactangle.height * reactangle.width
}
