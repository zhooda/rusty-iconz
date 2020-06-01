pub fn resize(src: &str, width: i32, height: i32, name: String, ipad: bool) {
    println!("{} -> {}", src, &name);
    let icname = if ipad == true { format!("{}~ipad.png", &name) } else { format!("{}.png", &name) };
    std::process::Command::new("convert")
        .arg(src)
        .arg("-resize")
        .arg(format!("{}x{}", width, height))
        .arg(icname)
        .output()
        .expect("Could not convert");
}