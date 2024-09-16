use image::imageops::FilterType;
use image::DynamicImage;
use webp::{Encoder, WebPMemory};

use log::{debug, error, info};

pub async fn get_image_data(
    img: &image::DynamicImage,
    format: image::ImageOutputFormat,
) -> Result<Vec<u8>, image::ImageError> {
    let mut data = Vec::new();
    match img.write_to(&mut std::io::Cursor::new(&mut data), format) {
        Ok(_) => Ok(data),
        Err(e) => {
            error!("Error writing image bytes to the vector!");
            Err(e)
        }
    }
}

pub async fn get_image(input_path: &str) -> Result<image::DynamicImage, image::ImageError> {
    match image::open(input_path) {
        Ok(img) => Ok(img),
        Err(e) => {
            error!("Error occurred in get_image: {:?}", e);
            Err(e)
        }
    }
}

pub async fn resize_image(
    input_path: &str,
    output_path: &str,
    new_width: u32,
    new_height: u32,
) -> Result<image::DynamicImage, image::ImageError> {
    //check if the resized image already exists
    if let Ok(existing_image) = image::open(output_path) {
        info!("Resized image already exists");
        return Ok(existing_image);
    }

    let img = image::open(input_path)?;

    //this means no input for resize was provided return the original image
    if new_height == 0 && new_width == 0 {
        info!("Resize not required as no input width and height provided");
        return Ok(img);
    }

    let final_width: u32 = if new_width == 0 {
        let ratio: f32 = img.width() as f32 / img.height() as f32;
        debug!("Aspect Ratio: {ratio}");
        (new_height as f32 * ratio) as u32
    } else {
        new_width
    };

    let final_height: u32 = if new_height == 0 {
        let ratio: f32 = img.height() as f32 / img.width() as f32;
        debug!("Aspect Ratio: {ratio}");
        (new_width as f32 * ratio) as u32
    } else {
        new_height
    };

    debug!("Resizing to height: {final_height} and width: {final_width}");

    // Resize the image
    let resized = img.resize(final_width, final_height, FilterType::Gaussian);
    // Save the resized image
    resized.save(output_path)?;

    Ok(resized)
}
