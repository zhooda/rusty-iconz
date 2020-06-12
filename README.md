# rusty-iconz

![ubuntu](https://img.shields.io/github/workflow/status/zhooda/rusty-iconz/ubuntu?label=ubuntu&logo=ubuntu&logoColor=white)
![macos](https://img.shields.io/github/workflow/status/zhooda/rusty-iconz/macos?label=macOS&logo=apple&logoColor=white)
![windows](https://img.shields.io/github/workflow/status/zhooda/rusty-iconz/windows?label=windows&logo=windows&logoColor=white)
![crates.io](https://img.shields.io/crates/v/iconz)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/zhooda/rusty-iconz/blob/master/LICENSE.md)

A simple CLI to generate all those pesky Xcode icon sizes from one single source file. This is the first project I've written in Rust so there will be plenty of bad code but that's okay :)

***NEW:*** iconz now has some decent error handling. No more thread panics!

## Features

- [x] Conversions without imagemagick
- [x] Proper error handling
- [ ] Export to packaged xcasset format
- [ ] Support for iMessage icons
- [ ] Support for scaling non-icon assets

## Install from Cargo

You must have [rust & cargo](https://www.rust-lang.org) installed before you can install iconz.

Install the latest published version iconz:

```bash
$ cargo install iconz
```

## Building from source

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Installing

Clone the repository
```bash
$ git clone https://github.com/zhooda/rusty-iconz.git
$ cd rusty-iconz
```

Build/Run for debug using Cargo
```bash
$ cargo run # with command line arguments OR
$ cargo build # to compile but not run

# The binary will be rusty-iconz/target/debug/iconz
```

Build for release
```bash
$ cargo build --release
$ cargo run --release -- -i appicon.png testdir
# The binary will be rusty-iconz/target/release/iconz
```

### Usage

If you used `cargo install` you can simply call it like any other command. The only command line arguments needed are `source image` and `icon directory`
```bash
$ iconz --magic sourceimage.png icondir
    iconz v0.2.2
    make xcode icons
    ＃blacklivesmatter https://www.blacklivesmatters.com

    ✔ appstore.png ➜ ./testdir/AppIcon20x20@2x
    ✔ appstore.png ➜ ./testdir/AppIcon20x20@3x
    ✔ appstore.png ➜ ./testdir/AppIcon29x29@2x
    ✔ appstore.png ➜ ./testdir/AppIcon29x29@3x
    ✔ appstore.png ➜ ./testdir/AppIcon40x40@2x
    ✔ appstore.png ➜ ./testdir/AppIcon40x40@3x
    ✔ appstore.png ➜ ./testdir/AppIcon60x60@2x
    ✔ appstore.png ➜ ./testdir/AppIcon60x60@3x
    # ...
    kthxbye;
```
#### Command line flags
| Flag        |                          Description                          |
| :---------- | :-----------------------------------------------------------: |
| no flag     |                      Uses -i by default                       |
| -i, --image | Uses CatmullRom algorithm for resizing |

## Contributing

Pull requests are welcome.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
