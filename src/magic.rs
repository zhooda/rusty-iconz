use crate::cli;

pub fn resize(src: String, dim: Vec<i32>, name: String, ipad: bool) {
    // println!("[MAGIC]: {} -> {}", &src, &name);
    cli::pretty(src.to_owned(), name.to_owned());
    let icname = if ipad == true { format!("{}~ipad.png", &name) } else { format!("{}.png", &name) };
    std::process::Command::new("convert")
        .arg(src)
        .arg("-resize")
        .arg(format!("{}x{}", &dim[0], &dim[1]))
        .arg(icname)
        .output()
        .expect("Could not convert");
}

pub fn scale(src: String, dim: Vec<i32>, name: String, scales: Vec<i32>, ipad: bool) {
    for scale in scales {
        let scale_string = if scale > 1 { format!("{}@{}x", &name, &scale)} else { format!("{}", &name) };
        resize(src.to_owned(), [&dim[0]*scale, &dim[1]*scale].to_vec(), scale_string, ipad);
    }
}

pub fn make_for_xcode(src: &str, dir: &str) {
    std::fs::remove_dir_all(dir).expect("Could not clear directory");
    std::fs::create_dir(dir).expect("Could not create directory");
    scale(src.to_owned(), [20, 20].to_vec(), format!("./{}/AppIcon20x20", dir), [2, 3].to_vec(), false);
    scale(src.to_owned(), [29, 29].to_vec(), format!("./{}/AppIcon29x29", dir), [2, 3].to_vec(), false);
    scale(src.to_owned(), [40, 40].to_vec(), format!("./{}/AppIcon40x40", dir), [2, 3].to_vec(), false);
    scale(src.to_owned(), [60, 60].to_vec(), format!("./{}/AppIcon60x60", dir), [2, 3].to_vec(), false);
    scale(src.to_owned(), [20, 20].to_vec(), format!("./{}/AppIcon20x20", dir), [1, 2].to_vec(), true);
    scale(src.to_owned(), [29, 29].to_vec(), format!("./{}/AppIcon29x29", dir), [1, 2].to_vec(), true);
    scale(src.to_owned(), [40, 40].to_vec(), format!("./{}/AppIcon40x40", dir), [1, 2].to_vec(), true);
    scale(src.to_owned(), [76, 76].to_vec(), format!("./{}/AppIcon76x76", dir), [1, 2].to_vec(), true);
    scale(src.to_owned(), [167, 167].to_vec(), format!("./{}/AppIcon83.5x83.5@2x", dir), [1].to_vec(), true);
    scale(src.to_owned(), [512, 512].to_vec(), format!("./{}/mac", dir), [1].to_vec(), false);
    scale(src.to_owned(), [1024, 1024].to_vec(), format!("./{}/AppStoreIcon", dir), [1].to_vec(), false);
    scale(src.to_owned(), [57, 57].to_vec(), format!("./{}/manifest/AppIcon57x57", dir), [1].to_vec(), false);
    scale(src.to_owned(), [512, 512].to_vec(), format!("./{}/manifest/AppIcon512x512", dir), [1].to_vec(), false);
}