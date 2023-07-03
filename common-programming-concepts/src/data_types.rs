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

    numeric_operations();
    
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


    // Compound types
    compound_types();

}


fn numeric_operations() -> () {
    // Numeric operations
    // addition
    let sum = 5 + 10;
    println!("addition");
    println!("{sum}");
    println!();

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference");
    println!("{difference}");
    println!();

    // multiplcation
    let product = 4 * 30;
    println!("product");
    println!("{product}");
    println!();

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient (literally division)");
    println!("{quotient}");
    println!();

    // remainder
    let remainder = 43 % 5;
    println!("remainder");
    println!("{remainder}");
    println!();
}

fn compound_types() -> () {
    // Tuples
    // General way of grouping multiple values into one type.
    // Once declared they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Then we use pattern matching to extract individual values from the tuple
    // destructuring
    let (x, y, z) = tup;
    println!("Our first ever tuple:");
    println!("0: {x}");
    println!("1: {y}");
    println!("2: {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Accessing the values directly
    println!("\nDirectly accessing the values");
    println!("0: {five_hundred}");
    println!("1: {six_point_four}");
    println!("2: {one}");

    // Empty tuples are called "units" -> ()
    // They represent an empty value or empty return type
    // This is implicitly returned if no return type specified


    // Arrays
    // An array must have all its values of the same type
    // AND arrays have a fixed length

    // example
    let a = [1, 2, 3, 4, 5];

    // arrays should mostly be used when you know that the collection of values will not have to change in size...
    // like the names of the months
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // statically typing an array:
    // [type, length]
    let b: [i32, 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]
    // this lets you specify the same value for the whole array

    // Accessing array elements
    let first = a[0]
    let second = a[1]

}