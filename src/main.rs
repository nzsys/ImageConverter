use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};
use image::{DynamicImage, ImageEncoder, ImageReader, ColorType};
use structopt::StructOpt;
use webp::Encoder;
use image::codecs::avif::AvifEncoder;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    input_dir: String,

    #[structopt(long)]
    output_dir: String,

    #[structopt(long)]
    format: String,
}

fn main() {
    let args = Cli::from_args();

    let entries: Vec<_> = fs::read_dir(&args.input_dir).unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_image_file(&entry.path()))
        .collect();

    let bar = Arc::new(ProgressBar::new(entries.len() as u64));
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    let results: Vec<_> = entries.into_par_iter()
        .map(|entry| {
            let path = entry.path();
            let result = process_image(&path, &args.output_dir, &args.format);
            bar.inc(1);
            result.map_err(|e| {
                eprintln!("Failed to process {}: {}", path.display(), e);
                e
            })
        })
        .collect();

    bar.finish_with_message("All images processed.");

    for result in results {
        if let Err(error) = result {
            eprintln!("Error: {:?}", error);
        }
    }
}

fn is_image_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "bmp" | "tiff"),
        None => false,
    }
}

fn process_image(path: &Path, output_dir: &str, format: &str) -> io::Result<PathBuf> {
    let img = ImageReader::open(path).and_then(|reader| reader.decode().map_err(|e| io::Error::new(io::ErrorKind::Other, e)))?;

    let img = img.thumbnail(1024, 1024);

    let output_file = match format {
        "webp" => {
            let webp_image = convert_to_webp(img);
            let output_file = Path::new(output_dir)
                .join(path.file_stem().unwrap())
                .with_extension("webp");
            fs::write(&output_file, webp_image)?;
            output_file
        }
        "avif" => {
            let avif_image = convert_to_avif(img, output_dir, path)?;
            avif_image
        }
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported format")),
    };

    Ok(output_file)
}

fn convert_to_webp(img: DynamicImage) -> Vec<u8> {
    let encoder = Encoder::from_image(&img).unwrap();
    encoder.encode(80.0).to_vec()
}

fn convert_to_avif(img: DynamicImage, output_dir: &str, path: &Path) -> io::Result<PathBuf> {
    let rgba_image = img.to_rgba8();
    let output_file = Path::new(output_dir)
        .join(path.file_stem().unwrap())
        .with_extension("avif");
    let file = File::create(&output_file)?;
    let writer = BufWriter::new(file);
    let encoder = AvifEncoder::new(writer);
    encoder.write_image(&rgba_image, rgba_image.width(), rgba_image.height(), ColorType::Rgba8.into()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(output_file)
}
