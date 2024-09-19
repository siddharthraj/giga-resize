use crate::{
    config::AppConfig, file_utils::FileUtils, image_cache::ImageCache, img_util::*, AppData,
    ImageParams,
};
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer};
use log::{debug, error, info, warn};
use tokio::sync::Mutex;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();
    info!("Fetching config.");
    let config = match AppConfig::load_config() {
        Ok(config) => config,
        Err(e) => {
            panic!("Error fetching the config! {:?}", e);
        }
    };

    let cache = ImageCache::new(config.cache_size);
    let bind_address = config.bind_address.clone();
    let file_utils = FileUtils::new(config.clone());

    let app_state = web::Data::new(AppData {
        cache: Mutex::new(cache),
        config,
        file_utils,
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
pub async fn img_org(path: web::Path<ImageParams>, data: web::Data<AppData>) -> HttpResponse {
    info!("Fetching original image");
    let params = path.into_inner();
    resize_and_respond(&params, &data).await
}

#[get("/img-resize/{width}/{height}/{file_name}")]
pub async fn img_resize(path: web::Path<ImageParams>, data: web::Data<AppData>) -> HttpResponse {
    info!("Fetching resized image");
    let params = path.into_inner();
    resize_and_respond(&params, &data).await
}

#[get("/img-scale/{width}/{file_name}")]
pub async fn img_scale(path: web::Path<ImageParams>, data: web::Data<AppData>) -> HttpResponse {
    info!("Fetching scaled image");
    let params = path.into_inner();
    resize_and_respond(&params, &data).await
}

pub async fn return_internal_error() -> HttpResponse {
    error!("Internal Server error returned!");
    HttpResponse::InternalServerError()
        .content_type(ContentType::html())
        .body("Unable to resize the image")
}

pub async fn return_image(
    params: &ImageParams,
    file_path: &str,
    data: &web::Data<AppData>,
) -> HttpResponse {
    debug!("Returning image");
    let mut cache = data.cache.lock().await;

    //trying from the cache first
    let cache_id = ImageCache::get_cache_id(params);
    if let Some(cached) = cache.get(&cache_id) {
        debug!("Returning cached image {cache_id}");
        return return_cached(cached.to_vec(), params).await; //return early if cached
    }

    debug!("Reading image {file_path} from file system");
    let content_type = params.get_content_type().unwrap_or("image".to_string());

    let bytes = tokio::fs::read(file_path).await; //TODO: try to avoid this

    //read
    match bytes {
        Ok(bytes) => {
            //store to cache
            cache.insert(cache_id, bytes.clone());
            HttpResponse::Ok().content_type(content_type).body(bytes)
        }
        Err(_) => return_internal_error().await,
    }
}

pub async fn return_cached(data: Vec<u8>, params: &ImageParams) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(params.get_content_type().unwrap_or("image".to_string()))
        .body(data)
}

pub async fn resize_and_respond(params: &ImageParams, data: &web::Data<AppData>) -> HttpResponse {
    info!("Resizing the image");
    let config = &data.config;
    let file_utils = &data.file_utils;

    if !file_utils.file_exists(&params.file_name).await {
        warn!(
            "File doesn't exist at the path {}/{}",
            &config.input_path, &params.file_name
        );
        return HttpResponse::NotFound().body("File not found!");
    }
    let full_path = file_utils.build_input_path(params);

    //if it is a request for the original image return here itself
    if params.width.is_none() && params.height.is_none() {
        info!("Original image requested");
        return_image(params, &full_path, data).await;
    }

    if let Ok(output_path) = file_utils.build_output_path(params).await {
        let output_path = file_utils.build_path(&output_path, &params.file_name);

        let height = params.height.unwrap_or(0);
        let width = params.width.unwrap_or(0);

        if let Ok(full_path) = resize_image(&full_path, &output_path, width, height, params).await {
            return_image(params, &full_path, data).await
        } else {
            return_internal_error().await
        }
    } else {
        return_internal_error().await
    }
}
