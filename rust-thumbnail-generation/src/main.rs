use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use image::{ImageBuffer, Rgba, imageops::FilterType};
use libheif_rs::{HeifContext, ColorSpace, LibHeif, RgbChroma};

// Define constants
const INPUT_DIR: &str = "/app/images";
const OUTPUT_DIR: &str = "/app/thumbnails";
const THUMB_SIZE: u32 = 100;

fn main() {
    let lib_heif = LibHeif::new();
    println!("--- Starting Rust Thumbnail Generation ---");
    let total_start = Instant::now();
    fs::create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");
    let paths = fs::read_dir(INPUT_DIR)
        .expect("Failed to read input directory")
        .filter_map(Result::ok);
    for entry in paths {
        let path = entry.path();
        if path.is_file() {
            let file_start = Instant::now();
            match process_file(&lib_heif, &path) {
                Ok(_) => {
                    let duration = file_start.elapsed();
                    println!(
                        "✅ Processed [{:?}]: {} ms",
                        path.file_name().unwrap_or_default(),
                        duration.as_millis()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "❌ Error processing [{:?}]: {}",
                        path.file_name().unwrap_or_default(),
                        e
                    );
                }
            }
        }
    }
    let total_duration = total_start.elapsed();
    println!("\n--- Finished in {:.2?} ---", total_duration);
}

fn process_file(lib_heif: &LibHeif, source_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let extension = source_path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    let file_stem = source_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
    let output_filename = format!("{}-thumbnail.png", file_stem);
    let destination_path = PathBuf::from(OUTPUT_DIR).join(output_filename);
    match extension.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "gif" => {
            let img = image::open(source_path)?;
            let thumbnail = img.resize_to_fill(THUMB_SIZE, THUMB_SIZE, FilterType::Lanczos3);
            thumbnail.save(destination_path)?;
        }
        "heic" | "heif" => {
            let ctx = HeifContext::read_from_file(source_path.to_str().unwrap())?;
            let handle = ctx.primary_image_handle()?;
            let heif_image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?;
            let width = heif_image.width();
            let height = heif_image.height();
            let planes = heif_image.planes();
            let interleaved = planes.interleaved.ok_or("Could not get interleaved plane")?;
            
            let img_buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, interleaved.data.to_vec())
                .ok_or("Could not create image buffer from raw data")?;
            
            let dynamic_image = image::DynamicImage::ImageRgba8(img_buffer);
            let thumbnail = dynamic_image.resize_to_fill(THUMB_SIZE, THUMB_SIZE, FilterType::Lanczos3);
            thumbnail.save(destination_path)?;
        }
        _ => { println!("- Skipping unsupported file: {:?}", source_path.file_name().unwrap_or_default()); }
    }
    Ok(())
}
