# Image Resizer

## Description

This project is a simple image resizing tool written in Rust. It allows you to resize an image by specifying the width, height, and whether to maintain the aspect ratio.

## Usage

First, compile the project by running the following command:

```sh
cargo build --release
```

The compiled binary will be available in the `target/release` folder.

To use the compiled executable, run the following command:

```sh
./target/release/image_resizer <image_path> <width> <height> <maintain_aspect>
```

- `<image_path>`: The path to the image to be resized.
- `<width>`: The new width of the image.
- `<height>`: The new height of the image.
- `<maintain_aspect>`: `true` to maintain the aspect ratio, `false` otherwise.

Example:

```sh
./target/release/image_resizer image.jpg 800 600 true
```

## Dependencies

This project uses the following dependencies:

- `image`: For image manipulation.
- `time`: To measure execution time.

## License

This project is released under the MIT license.
