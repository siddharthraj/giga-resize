use crate::config::AppConfig;
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer};
use image::ImageOutputFormat;
use log::{error, info, warn};
use tokio::sync::Mutex;

mod config;
mod file_utils;
mod image_cache;
mod img_util;
mod models;

use img_util::{get_image, get_image_data, resize_image};
use models::ImageParams;

struct AppData {
    cache: Mutex<image_cache::ImageCache>,
    config: AppConfig,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();
    info!("Fetching config.");
    let config = match AppConfig::load_config() {
        Ok(config) => config,
        Err(e) => {
            panic!("Error fetching the config! {:?}", e);
        }
    };

    let cache = image_cache::ImageCache::new(config.cache_size);
    let bind_address = config.bind_address.clone();

    let app_state = web::Data::new(AppData {
        cache: Mutex::new(cache),
        config,
    });

    let _ = HttpServer::new(move || {
        info!("🚀 Server Started");
        App::new()
            .app_data(app_state.clone())
            .service(img_org)
            .service(img_scale)
            .service(img_resize)
    })
    .bind(bind_address)?
    .run()
    .await;

    Ok(())
}

#[get("/img/{file_name}")]
async fn img_org(path: web::Path<ImageParams>, data: web::Data<AppData>) -> HttpResponse {
    info!("Fetching original image");
    let params = path.into_inner();
    resize_and_respond(&params, data).await
}

#[get("/img-resize/{width}/{height}/{file_name}")]
async fn img_resize(path: web::Path<ImageParams>, data: web::Data<AppData>) -> HttpResponse {
    info!("Fetching resized image");
    let params = path.into_inner();
    resize_and_respond(&params, data).await
}

#[get("/img-scale/{width}/{file_name}")]
async fn img_scale(path: web::Path<ImageParams>, data: web::Data<AppData>) -> HttpResponse {
    info!("Fetching scaled image");
    let params = path.into_inner();
    resize_and_respond(&params, data).await
}

async fn return_internal_error() -> HttpResponse {
    error!("Internal Server error returned!");
    HttpResponse::InternalServerError()
        .content_type(ContentType::html())
        .body("Unable to resize the image")
}

async fn return_image(img: &image::DynamicImage, params: &ImageParams) -> HttpResponse {
    let data = get_image_data(img, params.get_format().unwrap_or(ImageOutputFormat::Png)).await;

    match data {
        Ok(data) => HttpResponse::Ok()
            .content_type(params.get_content_type().unwrap_or("image".to_string()))
            .body(data),
        Err(_) => return_internal_error().await,
    }
}

async fn resize_and_respond(params: &ImageParams, data: web::Data<AppData>) -> HttpResponse {
    info!("Resizing the image");
    let cache_id = image_cache::ImageCache::get_cache_id(params);
    let mut cache = data.cache.lock().await;
    let config = &data.config;
    let format = params.get_format();

    if format.is_none() {
        warn!("Invalid file format received");
        return HttpResponse::NotFound().body("File not found!");
    }

    if let Some(image) = cache.get(cache_id.as_str()) {
        info!("Serving cached image");
        return return_image(image, params).await;
    }

    if !file_utils::file_exists(&config.input_path, &params.file_name) {
        warn!(
            "File doesn't exist at the path {}/{}",
            &config.input_path, &params.file_name
        );
        return HttpResponse::NotFound().body("File not found!");
    }

    //if it is a request for the original image return here itself
    if params.width.is_none() && params.height.is_none() {
        let img = get_image(params.get_image_path(&config.input_path).as_str()).await;
        match img {
            Ok(img) => {
                //store to the cache
                info!("Storing to {cache_id}");
                cache.insert(cache_id, img.clone());
                return return_image(&img, params).await;
            }
            Err(_) => {
                warn!("Unable to fetch the original image!");
                return return_internal_error().await;
            }
        }
    }

    if let Ok(output_path) = file_utils::build_output_path(&config.output_path, params) {
        let full_path = file_utils::build_input_path(&config.input_path, params);
        let output_path = file_utils::build_path(&output_path, &params.file_name);

        let height = params.height.unwrap_or(0);
        let width = params.width.unwrap_or(0);

        if let Ok(img) = resize_image(&full_path, &output_path, width, height).await {
            warn!("Unable to resize the image");
            cache.insert(cache_id, img.clone());
            return_image(&img, params).await
        } else {
            return_internal_error().await
        }
    } else {
        return_internal_error().await
    }
}
