use std::{error::Error, io::Cursor};

use image::ImageFormat;

pub fn resize_image(
    bytes: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    format: ImageFormat,
    maintain_aspect: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let image = image::load_from_memory_with_format(bytes, format)?;

    let original_width = image.width();
    let original_height = image.height();

    let resize_width = match width {
        Some(width) => width,
        None => match height {
            Some(height) => {
                if maintain_aspect {
                    (height as f32 / original_height as f32 * original_width as f32) as u32
                } else {
                    height
                }
            }
            None => return Err(Box::from("Must specify at least one of width or height")),
        },
    };

    let resize_height = match height {
        Some(height) => height,
        None => {
            if maintain_aspect {
                (resize_width as f32 * original_height as f32 / original_width as f32) as u32
            } else {
                resize_width
            }
        }
    };

    let resized_image = image.resize(
        resize_width,
        resize_height,
        image::imageops::FilterType::Nearest,
    );

    let mut buffer = Cursor::new(Vec::new());
    resized_image.write_to(&mut buffer, format)?;

    Ok(buffer.into_inner())
}
