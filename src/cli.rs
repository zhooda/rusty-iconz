// Writes message to stderr and returns error code
pub fn err(stderr: String, code: i32) {
    eprintln!("{}", stderr);
    std::process::exit(code);
}

// Collects command line arguments, verifies them,
// and returns a clone of them as a Vec<String>
pub fn verify_args(min: usize) -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < (min + 1) {
        err(format!("USAGE:\n    {} <source image> <icon directory>\n", &args[0]), -1);
    }
    return args.clone();
}

// Display some welcome info for the CLI
pub fn start(name: &str, version: &str, message: &str) {
    print!("\u{1F97a}{} v{}\n{}\n\n", &name, &version, &message);
}

// Display a goodbye message for the CLI
pub fn end() {
    println!("\nkthxbye;");
}