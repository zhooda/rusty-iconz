extern crate colored;
extern crate image;

use colored::*;
use image::ImageBuffer;

// Pretty printing
pub fn pretty(src: String, end: String) {
    println!("{} {} {} {}", "\u{2714}".bright_green(), src.italic(), "\u{279c}".bright_green(), end.bright_cyan().italic());
}

// A function that resizes an image buffer and saves it to a specified location
pub fn resize(buff: ImageBuffer<image::Rgb<u8>, Vec<u8>>, srcname: &str, width: u32, height: u32, name: String, ipad: bool) {
    let icname = if ipad == true { format!("{}~ipad.png", &name) } else { format!("{}.png", &name) };
    let resized = image::imageops::resize(&buff, width, height, image::imageops::FilterType::CatmullRom);
    resized.save(icname).expect("Could not save file");
    pretty(srcname.to_owned(), name.to_owned());
}

// A function that generates multiple scales of an icon
pub fn scale(buff: ImageBuffer<image::Rgb<u8>, Vec<u8>>, srcname: &str, width: u32, height: u32, name: String, scales: Vec<u32>, ipad: bool) {
    for scale in scales {
        let scale_string = if scale > 1 { format!("{}@{}x", &name, &scale)} else { format!("{}", &name) };
        resize(buff.to_owned(), srcname, width*scale, height*scale, scale_string, ipad);
    }
}

// A functino that generates all the icons necessary for an Xcode project
pub fn make_for_xcode(buff: ImageBuffer<image::Rgb<u8>, Vec<u8>>, srcname: &str, dir: &str) {
    std::fs::remove_dir_all(dir).expect("Could not empty directory");
    std::fs::create_dir(dir).expect("Could not create directory");
    std::fs::create_dir(format!("{}/manifest", dir)).expect("Could not create directory");

    scale(buff.to_owned(), srcname, 20, 20, format!("./{}/AppIcon20x20", dir), [2, 3].to_vec(), false);
    scale(buff.to_owned(), srcname, 29, 29, format!("./{}/AppIcon29x29", dir), [2, 3].to_vec(), false);
    scale(buff.to_owned(), srcname, 40, 40, format!("./{}/AppIcon40x40", dir), [2, 3].to_vec(), false);
    scale(buff.to_owned(), srcname, 60, 60, format!("./{}/AppIcon60x60", dir), [2, 3].to_vec(), false);
    scale(buff.to_owned(), srcname, 20, 20, format!("./{}/AppIcon20x20", dir), [1, 2].to_vec(), true);
    scale(buff.to_owned(), srcname, 29, 29, format!("./{}/AppIcon29x29", dir), [1, 2].to_vec(), true);
    scale(buff.to_owned(), srcname, 40, 40, format!("./{}/AppIcon40x40", dir), [1, 2].to_vec(), true);
    scale(buff.to_owned(), srcname, 76, 76, format!("./{}/AppIcon76x76", dir), [1, 2].to_vec(), true);
    scale(buff.to_owned(), srcname, 167, 167, format!("./{}/AppIcon83.5x83.5@2x", dir), [1].to_vec(), true);
    scale(buff.to_owned(), srcname, 512, 512, format!("./{}/mac", dir), [1].to_vec(), false);
    scale(buff.to_owned(), srcname, 1024, 1024, format!("./{}/AppStoreIcon", dir), [1].to_vec(), false);
    scale(buff.to_owned(), srcname, 57, 57, format!("./{}/manifest/AppIcon57x57", dir), [1].to_vec(), false);
    scale(buff.to_owned(), srcname, 512, 512, format!("./{}/manifest/AppIcon512x512", dir), [1].to_vec(), false);
}
