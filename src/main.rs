use crate::config::AppConfig;
use file_utils::FileUtils;

mod config;
mod file_utils;
mod image_cache;
mod image_server;
mod img_util;
mod models;

use models::AppData;
use models::ImageParams;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    match image_server::start_server().await {
        Ok(_) => Ok(()),
        Err(e) => panic!("Error starting the image server {:?}", e),
    }
}
