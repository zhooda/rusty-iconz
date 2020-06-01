pub fn resize(src: String, dim: Vec<i32>, name: String, ipad: bool) {
    println!("{} -> {}", &src, &name);
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