#![allow(unused_assignments)]
#![allow(unused_must_use)]

mod cli;
mod icon;


fn main() {
    cli::start("iconz", "0.1.1", "make those stupid xcode icons");

    let argv: Vec<String> = cli::verify_args(2);
    let src_img: &str = &argv[1];
    println!("argv: Vec<String> = {:?}", &argv);

    std::fs::remove_dir_all("./testdir");
    std::fs::create_dir("./testdir");
    icon::resize(src_img, 200, 200, format!("./testdir/200x200"), false);

    cli::end();
}
