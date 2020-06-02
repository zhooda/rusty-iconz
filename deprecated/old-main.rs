#![allow(unused_assignments)]
#![allow(unused_must_use)]

extern crate image;

mod cli;
mod icon;
mod native;

fn main() {

    let argv: Vec<String> = std::env::args().collect();

    // At least two arguments (source image and directory) must be specified
    if argv.len() < 3 {
        cli::usage(argv[0].clone());
    // The -m or --magic flag is used
    }
    
    if &argv[1] == "-m" || &argv[1] == "--magic" {
        let is_valid= std::path::Path::new(&argv[2]).exists();
        if is_valid {
            cli::start("iconz", "0.1.2", "make xcode icons\n#blacklivesmatter http://ally.wiki");
            icon::make_for_xcode(&argv[2], &argv[3]);
        } else {
            cli::err(format!("[iconz error]: {} is an invalid file path", &argv[2]), -1);
        }
    // The -i or --image flag is used
    } else if argv.len() == 4 {
        let is_valid= std::path::Path::new(&argv[2]).exists();
        if is_valid {
            cli::start("iconz", "0.1.2", "make xcode icons\n#blacklivesmatter http://ally.wiki");
            let buffer = image::open(&argv[2]).unwrap().into_rgb();
            native::make_for_xcode(buffer, &argv[2], &argv[3]);
        } else {
            cli::err(format!("[iconz error]: {} is an invalid file path", &argv[2]), -1);
        }
    // No flags are used -> Defaults to -i instead of -m
    } else {
        let is_valid= std::path::Path::new(&argv[1]).exists();
        if is_valid {
            cli::start("iconz", "0.1.2", "make xcode icons\n#blacklivesmatter http://ally.wiki");
            let buffer = image::open(&argv[1]).unwrap().into_rgb();
            native::make_for_xcode(buffer, &argv[1], &argv[2]);
        } else {
            cli::err(format!("[iconz error]: {} is an invalid file path", &argv[1]), -1);
        }
    }

    cli::end();
}
