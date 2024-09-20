use crate::models::ImageParams;
use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, PixelType, Resizer};
use image::{DynamicImage, ImageBuffer, ImageFormat, ImageReader, Rgb, Rgba};

use log::{debug, error, info, warn};

pub async fn get_image(
    input_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Sync + Send>> {
    info!("Getting image from: {input_path}");
    match tokio::fs::read(input_path).await {
        Ok(img) => Ok(img),
        Err(e) => {
            error!("Error occurred in get_image: {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn resize_image(
    input_path: &str,
    output_path: &str,
    new_width: u32,
    new_height: u32,
    params: &ImageParams,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    //check if the resized image already exists
    if get_image(output_path).await.is_ok() {
        info!("Resized image already exists");
        return Ok(output_path.to_string());
    }

    let img = ImageReader::open(input_path).unwrap().decode().unwrap();

    //this means no input for resize was provided return the original image
    if new_height == 0 && new_width == 0 && get_image(input_path).await.is_ok() {
        warn!("Original image reaching to the resizer!");
        return Ok(input_path.to_string());
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

    let mut output_image = Image::new(
        final_width,
        final_height,
        img.pixel_type().unwrap_or(PixelType::U8x4),
    );

    let mut resizer = Resizer::new();

    match resizer.resize(&img, &mut output_image, None) {
        Ok(_) => info!("Image resized"),
        Err(e) => {
            error!("Unable to resize the image {:?}", e);
            return Err(Box::new(e));
        }
    }

    if let Some(format) = params.get_format() {
        match format {
            ImageFormat::WebP => {
                info!("Encoding webp");
                let data = output_image.buffer_mut();
                let encoder = webp::Encoder::from_rgb(data, final_width, final_height);
                let webp: webp::WebPMemory = encoder.encode(50f32);
                info!("Writing output image to {output_path}");
                tokio::fs::write(&output_path, &*webp).await?;
                if get_image(output_path).await.is_ok() {
                    info!("Resized image already exists");
                    return Ok(output_path.to_string());
                }
            }
            ImageFormat::Png | ImageFormat::Jpeg => {
                info!("Encoding jpeg");

                let dyn_img = to_dyn_image(output_image);

                if dyn_img.is_none() {
                    error!("Unable to convert to dynamic image");
                    return Ok(output_path.to_string());
                }

                let file =
                    std::fs::File::create(output_path).expect("Unable to create output file");

                let mut buf = std::io::BufWriter::new(file);
                dyn_img
                    .unwrap()
                    .write_to(&mut buf, format)
                    .expect("Unable to write the image");
            }
            _ => {
                error!("Not supported!")
            }
        }
    }

    Ok(output_path.to_string())
}

fn to_dyn_image(image: Image) -> Option<DynamicImage>
where
{
    match image.pixel_type() {
        PixelType::U8x3 => {
            let buff_data = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
                image.width(),
                image.height(),
                image.buffer().to_vec(),
            );

            if buff_data.is_none() {
                error!("Unable to encode image!");
                return None;
            }
            let buffer = buff_data.unwrap();
            Some(DynamicImage::ImageRgb8(buffer))
        }
        PixelType::U8x4 => {
            let buff_data = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                image.width(),
                image.height(),
                image.buffer().to_vec(),
            );

            if buff_data.is_none() {
                error!("Unable to encode image!");
                return None;
            }
            let buffer = buff_data.unwrap();
            Some(DynamicImage::ImageRgba8(buffer))
        }
        _ => {
            error!("Not supported!");
            None
        }
    }
}
