# rusty-iconz

A simple CLI to generate all those pesky Xcode icon sizes from one single source file. This is the first project I've writtin in Rust so there will be plenty of bad code but that's okay :)

Currently, this project uses shell commands for Imagemagick to resize images; however, once I figure out how to use a Rust image manipulation library I'll get rid of the shell commands and Imagemagick dependency.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- [Rust 1.43.1](https://www.rust-lang.org)
- [Imagemagick](https://imagemagick.org/script/download.php)

### Installing 

Clone the repository
```bash
$ git clone https://github.com/zhooda/rusty-iconz.git
$ cd rusty-iconz
```

Build/Run for debug using Cargo
```bash
rusty-iconz$ cargo run # with command line arguments OR
rusty-iconz$ cargo build # to compile but not run

# The binary will be rusty-iconz/target/debug/iconz
```

Build for release (if you want to add it to your $PATH)
```bash
rusty-iconz$ cargo build --release
# The binary will be rusty-iconz/target/release/iconz
```

### Usage

If iconz is added to your $PATH env variable you can simply call it like any other command. The command line arguments needed are `source image` and `icon directory`
```bash
$ iconz sourceimage.png icondir
    sourceimage.png -> ./icondir/AppIcon20x20@2x
    sourceimage.png -> ./icondir/AppIcon20x20@3x
    sourceimage.png -> ./icondir/AppIcon29x29@2x
    sourceimage.png -> ./icondir/AppIcon29x29@3x
    sourceimage.png -> ./icondir/AppIcon40x40@2x
    sourceimage.png -> ./icondir/AppIcon40x40@3x
    ...
    kthxbye;
```

## Contributing

Pull requests are welcome.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details