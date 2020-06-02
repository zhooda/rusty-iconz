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