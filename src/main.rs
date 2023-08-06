use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{RgbImage, Rgb, Rgba, Rgba32FImage, Rgb32FImage};




// const IMAGE: &str = "rnd_images/AllHalfValues.exr";
// const BURNIN: &str = "render/burnin_uv_map_1280x720_32f_rgba_no_premult.exr";
const IMAGE: &str = "rnd_images/Desk.exr";
const BURNIN: &str = "render/burnin_solidcolour_644x874_32f_rgba_no_premult.exr";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let img = ImageReader::open(IMAGE)?.decode()?.to_rgba32f();
    let (width, height) = img.dimensions();
    let burnin = ImageReader::open(BURNIN)?.decode()?.to_rgba32f();
    let mut img_new = Rgb32FImage::new(width, height);


    for y in 0..height {
        for x in 0..width {
            let col = img.get_pixel(x, y);
            let bin = burnin.get_pixel(x, y);
            let new_col = [
                (bin[0] * bin[3]) + (col[0] * (1.0 - bin[3])),
                (bin[1] * bin[3]) + (col[1] * (1.0 - bin[3])),
                (bin[2] * bin[3]) + (col[2] * (1.0 - bin[3])),
            ];
            img_new.put_pixel(x, y, Rgb([new_col[0], new_col[1], new_col[2]]));
        }
    }
    img_new.save("result.exr")?;
    Ok(())
}
