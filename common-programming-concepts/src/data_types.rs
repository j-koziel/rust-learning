pub fn main() {
    // Two data subsets
    // scalar and compound

    // Since rust is a statically typed language it is a must to annotate each variable with a data type -> reliability

    /////////////////////////////////
    // Scalar types
    // These are types which represent a single value.
    // integers, floating-points, booleans and chars

    // Integer types
    // A number without a fractional component -> basically no decimals
    // There are unsigned and signed
    // Signed refers to integers which are negative and require a "sign"
    // Unsigned is when we are sure that the number can only ever be positive and does not require a "sign"

    // Limits of unsigned and signed integers
    // n is the number of bits
    // unsigned: 0 to (2^n) - 1

    // signed: -(2^n-1) -> 2^n-1 - 1

    // Floating-point types
    // There are only 2 primitive types for floating point numbers (f32 and f64).
    // When using type inference, it defaults to f64 because the speed of f32 and f64 is similar on modern CPUs
    // Also, all floating point numbers are signed.
    // example:
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric operations
    // addition
    let sum = 5 + 10;
    println!("addition");
    println!("{sum}");
    println!();

    let difference = 95.5 - 4.3;
    println!("difference");
    println!("{difference}");
    println!();

    let product = 4 * 30;
    println!("product");
    println!("{product}");
    println!();

    let quotient = 56.7 / 32.2;
    println!("quotient (literally division)");
    println!("{quotient}");
    println!();

    let remainder = 43 % 5;
    println!("remainder");
    println!("{remainder}");
    println!();

    
    // The boolean type
    // Basically like any other language -> two possible values of true and false
    // one byte in size
    let t = true;
    let f: bool = false;


    // The character type
    // char literals are specified with single quotes whereas string literal use double quotes
    // Interestingly it can store more than just ASCII -> emojis, kanji, karagana etc can also be used
    // Human intuition with what a character is may not match up with what rust sees chars as
    // Examples:
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
}
