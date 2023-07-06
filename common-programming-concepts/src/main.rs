mod data_types;
mod functions;
mod variables_and_mutability;

fn main() {
    println!("This is all about variables and mutability!");
    variables_and_mutability::main();

    new_section("data types");
    data_types::main();

    new_section("functions");
    functions::main();
}

fn new_section(subject: &str) {
    println!("\n#############################################");

    println!("This is all about {subject}!");
}
