extern crate colored;

// Display some welcome info for the CLI
pub fn start(name: &str, version: &str, message: &str) {
    print!("{} v{}\n{}\n\n", name, version, message);
}

// Display a goodbye message for the CLI
pub fn end() {
    println!("\nkthxbye;");
}
