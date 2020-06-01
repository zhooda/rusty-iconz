extern crate image;

// use std::io::File;

pub fn run(src: &str) {
    // let image = image::io::Reader::open(src)?.decode();
    // let copy = image::imageops::resize(&image, 200, 200, image::imageops::FilterType::Lanczos3);
    let orig = image::open("appstore.png").unwrap().into_rgb();
    let resized = image::imageops::resize(&orig, 200, 200, image::imageops::FilterType::Lanczos3);
    resized.save("edited.png");
}