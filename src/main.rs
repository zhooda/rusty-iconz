#![allow(unused_assignments)]
#![allow(unused_must_use)]

mod cli;
mod icon;

fn main() {
    cli::start("iconz", "0.1.1", "make those stupid xcode icons");

    let argv: Vec<String> = cli::verify_args(2);
    // let src_img: &str = &argv[1];
    // let dir = &argv[2];
    let (src_img, dir) = (&argv[1], &argv[2]);

    icon::make_for_xcode(src_img, dir);

    cli::end();
}
