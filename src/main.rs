// Functions take parameters, the values
// these parameters correspond to are called
// arguments
fn intiger(x: i32) {
    println!("The value of x is: {}", x);
}

// Functions with return values should have
// their type declared
fn five() -> i32 {
    5
}

fn main() {
    intiger(5);

    let x = five();

    println!("The value of x is: {}", x);
}
