# ImageConverter

ImageConverter is a Rust-based command-line tool to convert images from various formats (like JPEG, PNG, BMP, TIFF) to either WebP or AVIF format. The tool leverages parallel processing to efficiently handle multiple images, providing optimized performance for batch conversion tasks.

## Features

- Convert images from JPEG, PNG, BMP, TIFF formats to WebP or AVIF.
- Resize images to reduce file size and improve conversion speed.
- Parallel processing using Rayon for better performance.
- Progress bar with Indicatif to show conversion progress.

## Requirements

- Rust
- Cargo

## Installation

1. First, ensure you have Rust and Cargo installed. If not, you can install them via [rustup](https://rustup.rs/).

2. Clone this repository:
    ```sh
    git clone https://github.com/nzsys/ImageConverter.git
    cd ImageConverter
    ```

3. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

To use ImageConverter, run the following command with the appropriate arguments:

```sh
./target/release/image_converter --input-dir <input_dir> --output-dir <output_dir> --format <format>
```

### Arguments:

- `--input-dir <input_dir>`: Specifies the directory containing images to be converted.
- `--output-dir <output_dir>`: Specifies the directory where the converted images will be saved.
- `--format <format>`: Specifies the output format for the images. Supported formats: `webp`, `avif`.

### Example:

```sh
./target/release/image_converter --input-dir ./input --output-dir ./output --format avif
```

This command will convert all supported images in the `./input` directory to the AVIF format and save them in the `./output` directory.

## Contributing

Contributions are welcome! Feel free to open an issue or a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License.