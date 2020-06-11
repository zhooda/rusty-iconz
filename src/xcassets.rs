use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct IdiomContents {
    idiom: String,
    scale: String,
    size: String,2
}

#[derive(Serialize, Deserialize)]
pub struct AppIconContents {
    x: i32,
    images: Vec<IdiomContents>,
    info: AssetsSubcontents,
}

#[derive(Serialize, Deserialize)]
pub struct AssetsSubcontents {
    author: String,
    version: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AssetsContents {
    info: AssetsSubcontents,
}
