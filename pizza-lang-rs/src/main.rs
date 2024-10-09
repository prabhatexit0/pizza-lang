use std::env;
use std::fs;

mod lexer;
use lexer::Lexer;

fn main() -> std::io::Result<()> {
    // Get the current working directory
    let mut path = env::current_dir()?;
    
    // Add the path to the examples directory
    path.push("examples");
    path.push("slices-per-person.pizza");

    println!("Attempting to read file: {:?}", path);

    // Read the file
    let contents = fs::read_to_string(path)?;
    println!("File contents:\n{}", contents);

    let mut lexer = Lexer::new(contents);
    let _ = lexer.get_all_tokens();
    println!("Tokens written to tokens.txt");

    Ok(())
}
