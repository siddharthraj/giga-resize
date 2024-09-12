use image::imageops::FilterType;

pub async fn get_image_data(
    img: &image::DynamicImage,
    format: image::ImageOutputFormat,
) -> Result<Vec<u8>, image::ImageError> {
    let mut data = Vec::new();
    match img.write_to(&mut std::io::Cursor::new(&mut data), format) {
        Ok(_) => Ok(data),
        Err(e) => Err(e),
    }
}

pub async fn get_image(input_path: &str) -> Result<image::DynamicImage, image::ImageError> {
    let img = image::open(input_path)?;
    Ok(img)
}

pub async fn resize_image(
    input_path: &str,
    output_path: &str,
    new_width: u32,
    new_height: u32,
) -> Result<image::DynamicImage, image::ImageError> {
    //check if the resized image already exists

    if let Ok(existing_image) = image::open(output_path) {
        println!("Resized image already exists");
        return Ok(existing_image);
    }

    let img = image::open(input_path)?;

    //this means no input for resize was provided return the original image
    if new_height == 0 && new_width == 0 {
        return Ok(img);
    }

    let final_width: u32 = if new_height == 0 {
        let ratio: f32 = img.width() as f32 / img.height() as f32;
        println!("ratio: {ratio}");
        (new_height as f32 * ratio) as u32
    } else {
        new_width
    };

    let final_height: u32 = if new_height == 0 {
        let ratio: f32 = img.height() as f32 / img.width() as f32;
        println!("ratio: {ratio}");
        (new_width as f32 * ratio) as u32
    } else {
        new_height
    };

    println!("New height: {final_height}");

    // Resize the image
    let resized = img.resize(final_width, final_height, FilterType::Lanczos3);

    // Save the resized image
    resized.save(output_path)?;

    Ok(resized)
}
