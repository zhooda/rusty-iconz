extern crate colored;
extern crate structopt;

#[macro_use] extern crate log;

use colored::*;
use structopt::StructOpt;

mod magic;
mod native;
mod cli;

#[derive(StructOpt)]
struct Options {
    source: String, // [1]
    directory: String, // [2]

    #[structopt(short = "m", long = "magic")]
    /// Uses Imagemagick for conversions and requires Imagemagick to be installed (slower)
    magic: bool,

    #[structopt(short = "i", long = "image")]
    /// Uses the CatmullRoll algorithm and doesnt require any dependencies (faster)
    image: bool,
}

fn main() {
    // Get all command line options
    let options = Options::from_args();
    let source = options.source;
    let directory = options.directory;
    let magic = options.magic;
    let image = options.image;

    let is_valid= std::path::Path::new(&source).exists();
    if !is_valid {
        let errmsg = format!("error:");
        eprintln!("{} The file path {} was invalid", errmsg=errmsg.red().bold(), src=source.clone().blue().underline());
        std::process::exit(-1);
    }

    if magic {
        cli::start("iconz", "0.2.0", "make xcode icons\n#blacklivesmatter http://ally.wiki");
        magic::make_for_xcode(&source, &directory);
    } else {
        cli::start("iconz", "0.1.2", "make xcode icons\n#blacklivesmatter http://ally.wiki");
            let buffer = image::open(&source).unwrap().into_rgb();
            native::make_for_xcode(buffer, &source, &directory);
    }

    cli::end();
}

#[cfg(test)]
mod iconz_tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let _buffer = image::open("./tests/test-icon.png").unwrap().into_rgb();
    }

    #[test]
    fn test_resize() {
        let assert_value: (u32, u32) = (200, 200);
        let buffer = image::open("./tests/test-icon.png").unwrap().into_rgb();
        native::resize(buffer, "./tests/test-icon.png", 200, 200, "./tests/test-icon-resized".to_owned(), false);
        let resized_buffer = image::open("./tests/test-icon-resized.png").unwrap().into_rgb();
        assert_eq!(assert_value, resized_buffer.dimensions());
    }

    #[test]
    fn test_name() {
        let buffer = image::open("./tests/test-icon.png").unwrap().into_rgb();
        native::resize(buffer.clone(), "./tests/test-icon.png", 200, 200, "./tests/test-icon-name".to_owned(), false);
        native::resize(buffer, "./tests/test-icon.png", 200, 200, "./tests/test-icon-name".to_owned(), true);
        let iphone_valid = std::path::Path::new("./tests/test-icon-name.png").exists();
        let ipad_valid = std::path::Path::new("./tests/test-icon-name~ipad.png").exists();
        assert_eq!(true, iphone_valid);
        assert_eq!(true, ipad_valid);
    }
}
