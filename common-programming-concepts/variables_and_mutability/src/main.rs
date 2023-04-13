fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    // the data type of constants is required to be annotated.
    // Only expressions can be assigned to constants, and not values which are computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing 🤯
    // basically, you can use the same name for a variable and you won't get errors
    let spaces: &str = "    ";
    println!("{spaces}");
    
    let spaces: usize = spaces.len();
    println!("{spaces}");

}
