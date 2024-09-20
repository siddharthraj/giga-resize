use image::ImageFormat;
use serde::Deserialize;
use std::path::Path;
use tokio::sync::Mutex;

use crate::{image_cache, AppConfig, FileUtils};

pub struct AppData {
    pub cache: Mutex<image_cache::ImageCache>,
    pub config: AppConfig,
    pub file_utils: FileUtils,
}

#[derive(Deserialize)]
pub struct ImageParams {
    pub file_name: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ImageParams {
    pub fn get_format(&self) -> Option<ImageFormat> {
        let ext = Path::new(&self.file_name)
            .extension()
            .and_then(std::ffi::OsStr::to_str);

        match ext {
            Some(ext) => match ext {
                "png" => Some(ImageFormat::Png),
                "jpeg" | "jpg" => Some(ImageFormat::Jpeg),
                "webp" => Some(ImageFormat::WebP),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_content_type(&self) -> Option<String> {
        let ext = Path::new(&self.file_name)
            .extension()
            .and_then(std::ffi::OsStr::to_str);

        match ext {
            Some(ext) => match ext {
                "png" => Some(String::from("image/png")),
                "jpeg" | "jpg" => Some(String::from("image/jpeg")),
                "webp" => Some(String::from("image/webp")),
                _ => None,
            },
            None => None,
        }
    }
}
