#![allow(unused_assignments)]

mod cli;
mod icon;

fn main() {
    cli::start("iconz", "0.1.1", "make those stupid xcode icons");

    let argv: Vec<String> = cli::verify_args(2);
    let src_img: &str = &argv[1];
    println!("argv: Vec<String> = {:?}", &argv);

    cli::end();
}
