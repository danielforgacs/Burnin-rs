use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{RgbImage, Rgba, Rgba32FImage};




const IMAGE: &str = "test_images/black_bg_linear_rgb_32bit_float.exr";
const BURNIN: &str = "test_images/burnin0.exr";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut img = ImageReader::open(IMAGE)?.decode()?.to_rgba32f();
    let mut img_new = Rgba32FImage::new(800, 400);


    for x in 0..800 {
        for y in 0..400 {
            // img_new.put_pixel(x, y, Rgba([0 as f32, (x % 2) as f32, y as f32, 0.0]));
            img_new.put_pixel(x, y, Rgba([1.0, 0.5, 0.0, 1.0]));
        }
    }
    img_new.save("test_images/black_bg_linear_rgb_32bit_float_new.exr")?;
    Ok(())
}
