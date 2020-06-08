# rusty-iconz

![ubuntu](https://img.shields.io/github/workflow/status/zhooda/rusty-iconz/ubuntu?label=ubuntu&logo=ubuntu&logoColor=white)
![macos](https://img.shields.io/github/workflow/status/zhooda/rusty-iconz/macos?label=macOS&logo=apple&logoColor=white)
![windows](https://img.shields.io/github/workflow/status/zhooda/rusty-iconz/windows?label=windows&logo=windows&logoColor=white)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=plastic&logo=appveyor)](https://github.com/zhooda/rusty-iconz/blob/master/LICENSE.md)

A simple CLI to generate all those pesky Xcode icon sizes from one single source file. This is the first project I've written in Rust so there will be plenty of bad code but that's okay :)

Iconz now uses the [image](https://crates.io/crates/image) crate to resize images using the CatmullRom algorithm. If you have Imagemagick installed and would like to use it instead, just use the ```-m``` or ```--magic``` flags.

***WARNING:*** There is currently no error handling which means if you enter an invalid file path, the program will crash

## Install a pre-compiled binary

If you want to install a pre-compiled binary, go to the [```bin/```](https://github.com/zhooda/rusty-iconz/tree/master/bin) folder and download the correct binary for your platform.

## Building from source

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- [Rust 1.43.1](https://www.rust-lang.org)
- [image](https://crates.io/crates/image) - A rust image encoding/decoding/manipulation library

### Dependencies
- **OPTIONAL** [Imagemagick](https://imagemagick.org/script/download.php)

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
$ iconz --magic sourceimage.png icondir
    sourceimage.png -> ./icondir/AppIcon20x20@2x
    sourceimage.png -> ./icondir/AppIcon20x20@3x
    sourceimage.png -> ./icondir/AppIcon29x29@2x
    sourceimage.png -> ./icondir/AppIcon29x29@3x
    sourceimage.png -> ./icondir/AppIcon40x40@2x
    sourceimage.png -> ./icondir/AppIcon40x40@3x
    ...
    kthxbye;
```
#### Command line flags
| Flag        |                          Description                          |
| :---------- | :-----------------------------------------------------------: |
| no flag     |                      Uses -i by default                       |
| -i, --image | Uses Rust *image* crate and CatmullRom algorithm for resizing |
| -m, --magic |                 Uses Imagemagick for resizing                 |

## Contributing

Pull requests are welcome.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
