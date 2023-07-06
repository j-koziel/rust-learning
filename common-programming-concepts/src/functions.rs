pub fn main() {
    println!("Hello, world!");
    let x = five();

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    // In rust you can have statements and expressions
    // An expression can look like this:
    let y = {
      let x = 3
      x + 1
    }
    println!("The value of x is: {x}")
}

// specifying the return type of the function
fn five() -> i32 {
    // This function returns 5
    // This is a perfectly normal function in rust
    5
}

// statements are simply instructions which do not return a value
// function definitions are statements
// expressions actually evaluate to a resultant value