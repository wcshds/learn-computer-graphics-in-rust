use image::DynamicImage;
use learn_computer_graphics_in_rust::image_processing::bayer::{
    cast_rgb_to_bayer_mosaic, demosaic, demosaic_rayon,
};

fn main() {
    let img = if let DynamicImage::ImageRgb8(content) = image::open("./resources/scan.png").unwrap()
    {
        content
    } else {
        panic!("Cannot read the image correctly.")
    };
    let gray = cast_rgb_to_bayer_mosaic(&img);
    gray.save("./resources/scan_bayer_mosaic.png").unwrap();

    let rgb = demosaic_rayon(&gray);
    rgb.save("./resources/scan_bayer_demosaic.png").unwrap();
}
