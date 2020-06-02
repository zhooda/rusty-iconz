// Writes message to stderr and returns error code
pub fn err(stderr: String, code: i32) {
    eprintln!("{}", stderr);
    std::process::exit(code);
}

// Writes the usage message to stderr
pub fn usage(prog: String) {
    err(format!("USAGE:\n    {} [flags] <source image> <icon directory>\n\nFLAGS:\n    -m, --magic ...... convert using imagemagick (default)\n    -i, --image ...... convert using rust image crate (slower)\n", prog), -1);
}

// Display some welcome info for the CLI
pub fn start(name: &str, version: &str, message: &str) {
    print!("\u{1F97a}{} v{}\n{}\n\n", &name, &version, &message);
}

// Display a goodbye message for the CLI
pub fn end() {
    println!("\nkthxbye;");
}