// function definitions are themselves statements
fn five() -> i32 {
    5
}

fn main() {
    // a statement performs some action without 
    // returning a value
    let _y = 6;

    // an expression evaluates to some value;
    // ex. calling a function or macro, the value
    // assigned in a statement, the {} braces
    let _y = {
        let x = 3;
        x + 1 // note the lack of semicolon; 
              // adding one turns it into a statement,
              // which means it will no longer return a value
    };
   

    let x = five();

    println!("The value of x is {}", x);
}
