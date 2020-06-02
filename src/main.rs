#![allow(unused_assignments)]
#![allow(unused_must_use)]

extern crate image;

mod cli;
mod icon;
mod native;

fn main() {
    cli::start("iconz", "0.1.2", "make xcode icons\n#blacklivesmatter http://ally.wiki");

    let argv: Vec<String> = std::env::args().collect();

    // At least two arguments (source image and directory) must be specified
    if argv.len() < 3 {
        cli::usage(argv[0].clone());
    // The -m or --magic flag is used
    }
    
    if &argv[1] == "-m" || &argv[1] == "--magic" {
        icon::make_for_xcode(&argv[2], &argv[3]);
    // The -i or --image flag is used
    } else if argv.len() == 4 {
        let buffer = image::open(&argv[2]).unwrap().into_rgb();
        native::make_for_xcode(buffer, &argv[2], &argv[3]);
    // No flags are used -> Defaults to -i instead of -m
    } else {
        let buffer = image::open(&argv[1]).unwrap().into_rgb();
        native::make_for_xcode(buffer, &argv[1], &argv[2]);
    }

    cli::end();
}
