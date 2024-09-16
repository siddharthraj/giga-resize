use crate::file_utils;
use image::ImageOutputFormat;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct ImageParams {
    pub file_name: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ImageParams {
    pub fn get_image_path(&self, path: &str) -> String {
        file_utils::build_input_path(path, self)
    }

    pub fn get_format(&self) -> Option<ImageOutputFormat> {
        let ext = Path::new(&self.file_name)
            .extension()
            .and_then(std::ffi::OsStr::to_str);

        match ext {
            Some(ext) => match ext {
                "png" => Some(ImageOutputFormat::Png),
                "jpeg" | "jpg" => Some(ImageOutputFormat::Jpeg(0)),
                "webp" => Some(ImageOutputFormat::WebP),
                "gif" => Some(ImageOutputFormat::Gif),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_content_type(&self) -> Option<String> {
        match self.get_format() {
            Some(format) => match format {
                ImageOutputFormat::Png => Some(String::from("image/png")),
                ImageOutputFormat::Jpeg(0) => Some(String::from("image/jpeg")),
                ImageOutputFormat::WebP => Some(String::from("image/webp")),
                ImageOutputFormat::Gif => Some(String::from("image/gif")),
                _ => None,
            },
            None => None,
        }
    }
}
