use std::io::Cursor;
use image::io::Reader as ImageReader;

const IMAGE: &str = "test_images/black_bg_linear_rgb_32bit_float.exr";
const BURNIN: &str = "test_images/burnin0.exr";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let img = ImageReader::open(IMAGE)?.decode()?;
    let burnin = ImageReader::open(BURNIN)?.decode()?;
    Ok(())
}
