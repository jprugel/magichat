use iced::advanced::image as iced_image;
use image::{GenericImageView, ImageReader};
use std::hash::{DefaultHasher, Hasher};
use std::io::Cursor;

pub async fn get_server_icon(url: &str) -> Result<iced_image::Handle, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let img = ImageReader::new(Cursor::new(&bytes))
        .with_guessed_format()?
        .decode()?;

    let pixels = img.to_rgba8().into_raw();

    Ok(iced_image::Handle::from_bytes(pixels))
}
