use crate::ImageParams;
use std::fs::File;

const IMAGE_SOURCE_PATH: &str = "/home/siddharth/NextCloud/rhwg/code/bs-hindi/Aut@ui1287";
const OUTPUT_PATH: &str = "/home/siddharth/NextCloud/rhwg/code/bs-hindi/Aut@ui1287";

pub fn file_exists(file_name: &str) -> bool {
    let mut full_path = IMAGE_SOURCE_PATH.to_string();
    full_path = build_path(&full_path, file_name);

    println!("{full_path}");

    let file = File::open(&full_path);
    file.is_ok()
}

pub fn build_output_path(params: &ImageParams) -> Result<String, Box<dyn std::error::Error>> {
    let mut output_path = String::from(OUTPUT_PATH);

    let height = params.height.unwrap_or(0);
    let width = params.width.unwrap_or(0);

    output_path = build_path(
        &output_path,
        &format!("{}{}{}", width, std::path::MAIN_SEPARATOR, height),
    );

    match std::fs::DirBuilder::new()
        .recursive(true)
        .create(&output_path)
    {
        Ok(_) => Ok(output_path),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn build_input_path(params: &ImageParams) -> String {
    let mut final_path: String = String::from(IMAGE_SOURCE_PATH);
    if !IMAGE_SOURCE_PATH.ends_with(std::path::MAIN_SEPARATOR) {
        final_path.push(std::path::MAIN_SEPARATOR);
    }
    final_path.push_str(&params.file_name);
    final_path
}

pub fn build_path(path: &str, file_name: &str) -> String {
    let mut final_path: String = String::from(path);
    if !path.ends_with(std::path::MAIN_SEPARATOR) {
        final_path.push(std::path::MAIN_SEPARATOR);
    }
    final_path.push_str(file_name);
    final_path
}
