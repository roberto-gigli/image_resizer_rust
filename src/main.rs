mod image_resizer_lib;
use std::{env, time::Instant};

fn main() {
    println!("Image resizer v{}", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();

    let image_path = match args.get(1) {
        Some(image_path) => image_path,
        None => panic!("Must provide image path"),
    };
    let width_string = match args.get(2) {
        Some(width_string) => width_string,
        None => panic!("Must provide width"),
    };
    let height_string = match args.get(3) {
        Some(height_string) => height_string,
        None => panic!("Must provide height"),
    };
    let maintain_aspect_string = match args.get(4) {
        Some(maintain_aspect_string) => maintain_aspect_string,
        None => panic!("Must provide maintain_aspect"),
    };

    let width = match width_string.parse::<u32>() {
        Ok(width) => Some(width),
        Err(_) => panic!("Width must be a number"),
    };

    let height = match height_string.parse::<u32>() {
        Ok(height) => Some(height),
        Err(_) => panic!("Height must be a number"),
    };

    let maintain_aspect = match maintain_aspect_string.as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("maintain_aspect must be true or false"),
    };

    let bytes = match std::fs::read(image_path) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Could not read image"),
    };

    let image_format = match image::ImageFormat::from_path(image_path) {
        Ok(format) => format,
        Err(_) => panic!("Could not determine image format"),
    };

    let start_time = Instant::now();
    let result =
        image_resizer_lib::resize_image(&bytes, width, height, image_format, maintain_aspect);
    let duration = start_time.elapsed();

    println!("Time taken: {:?}", duration);

    let extension = match image_format.extensions_str().first() {
        Some(extension) => extension,
        None => panic!("Could not determine image extension"),
    };

    match result {
        Ok(bytes) => {
            let output_path = &format!("resized_image.{}", extension);
            match std::fs::write(output_path, bytes) {
                Ok(_) => println!("Image saved to {}", output_path),
                Err(_) => println!("Could not save image"),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
