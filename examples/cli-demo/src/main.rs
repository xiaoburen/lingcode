// This file serves as the entry point for the CLI demo of the input method framework.
// It initializes the application and handles user input for Pinyin and Double Pinyin.

fn main() {
    println!("Welcome to the LingCode Rime CLI Demo!");
    println!("Please enter your input:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Here you would typically process the input using the core engine and libraries
    // For demonstration purposes, we will just echo the input back
    println!("You entered: {}", input.trim());

    // Further processing for Pinyin and Double Pinyin would go here
}