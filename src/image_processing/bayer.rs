use image::{GenericImageView, GrayImage, Rgb, RgbImage};
use rayon::iter::ParallelIterator;

/// Convert a normal RGB image to a Bayer color filter array.
pub fn cast_rgb_to_bayer_mosaic(img: &RgbImage) -> GrayImage {
    let bayer_gray_raw = img
        .enumerate_pixels()
        .map(|(x, y, &Rgb([red, green, blue]))| match (x % 2, y % 2) {
            (0, 0) | (1, 1) => green,
            (1, 0) => blue,
            (0, 1) => red,
            _ => unreachable!(),
        });

    let gray = GrayImage::from_vec(img.width(), img.height(), bayer_gray_raw.collect())
        .expect("Fail to convert RGB image to Gray.");

    gray
}

trait ExtIndexTrait<const D: usize> {
    /// Get an extended index value at coordinates (x, y).
    fn ext_index(&self, x: i32, y: i32) -> [u8; D];
    /// Convert out-of-bounds indices to valid in-bounds indices.
    fn convert_index(&self, x: i32, y: i32, width: u32, height: u32) -> (u32, u32) {
        let (width_i32, height_i32) = (width as i32, height as i32);
        let (right_bound, bottom_bound) = ((width - 1) as i32, (height - 1) as i32);

        let (actual_x, actual_y) = if x == -1 && y == -1 {
            // top-left edge
            (1, 1)
        } else if x == width_i32 && y == -1 {
            // top-right edge
            (width - 2, 1)
        } else if x == width_i32 && y == height_i32 {
            // bottom-right edge
            (width - 2, height - 2)
        } else if x == -1 && y == height_i32 {
            // bottom-left edge
            (1, height - 2)
        } else if y == -1 {
            // top-line edge
            (x as u32, 1)
        } else if y == height_i32 {
            // bottom-line edge
            (x as u32, height - 2)
        } else if x == -1 {
            // left-line edge
            (1, y as u32)
        } else if x == width_i32 {
            // right-line edge
            (width - 2, y as u32)
        } else if (0..=right_bound).contains(&x) && (0..=bottom_bound).contains(&y) {
            // on image
            (x as u32, y as u32)
        } else {
            panic!("Index error: index range should be [-1, width] x [-1, height], current index is {{x: {}, y: {}}}", x, y);
        };

        (actual_x, actual_y)
    }
}

impl ExtIndexTrait<3> for RgbImage {
    fn ext_index(&self, x: i32, y: i32) -> [u8; 3] {
        let (width, height) = self.dimensions();
        let (actual_x, actual_y) = self.convert_index(x, y, width, height);

        unsafe { self.unsafe_get_pixel(actual_x, actual_y).0 }
    }
}

impl ExtIndexTrait<1> for GrayImage {
    fn ext_index(&self, x: i32, y: i32) -> [u8; 1] {
        let (width, height) = self.dimensions();
        let (actual_x, actual_y) = self.convert_index(x, y, width, height);

        unsafe { self.unsafe_get_pixel(actual_x, actual_y).0 }
    }
}

/// There are a lot of demosaic algorithms. Here is just a simplified one.
pub fn demosaic(img: &GrayImage) -> RgbImage {
    let (width, height) = img.dimensions();
    let (width_i32, height_i32) = (width as i32, height as i32);
    let mut rgb_image = RgbImage::new(width, height);

    for y in 0..height_i32 {
        for x in 0..width_i32 {
            let red = if (y % 2 == 1) && (x % 2 == 0) {
                unsafe { img.unsafe_get_pixel(x as u32, y as u32).0[0] }
            } else {
                0
            };

            let blue = if (y % 2 == 0) && (x % 2 == 1) {
                unsafe { img.unsafe_get_pixel(x as u32, y as u32).0[0] }
            } else {
                0
            };

            let green = if (y % 2 == 0) && (x % 2 == 0) || (y % 2 == 1) && (x % 2 == 1) {
                unsafe { img.unsafe_get_pixel(x as u32, y as u32).0[0] }
            } else {
                let left = img.ext_index(x - 1, y)[0];
                let right = img.ext_index(x + 1, y)[0];
                let up = img.ext_index(x, y - 1)[0];
                let down = img.ext_index(x, y + 1)[0];

                ((left as u16 + right as u16 + up as u16 + down as u16) / 4) as u8
            };

            rgb_image.put_pixel(x as u32, y as u32, Rgb([red, green, blue]));
        }
    }

    for y in 0..(height_i32) {
        for x in 0..(width_i32) {
            let (x_mod, y_mod) = (x % 2, y % 2);
            match (x_mod, y_mod) {
                // origin green only
                (0, 0) | (1, 1) => {
                    let left = rgb_image.ext_index(x - 1, y);
                    let right = rgb_image.ext_index(x + 1, y);
                    let up = rgb_image.ext_index(x, y - 1);
                    let down = rgb_image.ext_index(x, y + 1);

                    let green = rgb_image.ext_index(x, y)[1];

                    let (blue, red) = if x_mod == 0 && y_mod == 0 {
                        // orgin top-left green only
                        let blue = ((left[2] as f32 / left[1] as f32
                            + right[2] as f32 / right[1] as f32)
                            / 2.0
                            * green as f32) as u8;
                        let red = ((up[0] as f32 / up[1] as f32 + down[0] as f32 / down[1] as f32)
                            / 2.0
                            * green as f32) as u8;

                        (blue, red)
                    } else {
                        // orgin bottom-right green only
                        let blue = ((up[2] as f32 / up[1] as f32 + down[2] as f32 / down[1] as f32)
                            / 2.0
                            * green as f32) as u8;
                        let red = ((left[0] as f32 / left[1] as f32
                            + right[0] as f32 / right[1] as f32)
                            / 2.0
                            * green as f32) as u8;

                        (blue, red)
                    };

                    rgb_image.put_pixel(x as u32, y as u32, Rgb([red, green, blue]));
                }
                (1, 0) | (0, 1) => {
                    let up_left = rgb_image.ext_index(x - 1, y - 1);
                    let up_right = rgb_image.ext_index(x + 1, y - 1);
                    let down_left = rgb_image.ext_index(x - 1, y + 1);
                    let down_right = rgb_image.ext_index(x + 1, y + 1);

                    let green = rgb_image.ext_index(x, y)[1];

                    let (blue, red) = if x_mod == 1 {
                        // origin blue only
                        let blue = rgb_image.ext_index(x, y)[2];
                        let red = ((up_left[0] as f32 / up_left[1] as f32
                            + up_right[0] as f32 / up_right[1] as f32
                            + down_left[0] as f32 / down_left[1] as f32
                            + down_right[0] as f32 / down_right[1] as f32)
                            / 4.0
                            * green as f32) as u8;

                        (blue, red)
                    } else {
                        // origin red only
                        let red = rgb_image.ext_index(x, y)[0];
                        let blue = ((up_left[2] as f32 / up_left[1] as f32
                            + up_right[2] as f32 / up_right[1] as f32
                            + down_left[2] as f32 / down_left[1] as f32
                            + down_right[2] as f32 / down_right[1] as f32)
                            / 4.0
                            * green as f32) as u8;

                        (blue, red)
                    };

                    rgb_image.put_pixel(x as u32, y as u32, Rgb([red, green, blue]));
                }
                _ => unreachable!(),
            }
        }
    }

    rgb_image
}

pub fn demosaic_rayon(img: &GrayImage) -> RgbImage {
    let (width, height) = img.dimensions();
    let mut rgb_image = RgbImage::new(width, height);

    rgb_image
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let x = x as i32;
            let y = y as i32;

            let red = if (y % 2 == 1) && (x % 2 == 0) {
                unsafe { img.unsafe_get_pixel(x as u32, y as u32).0[0] }
            } else {
                0
            };

            let blue = if (y % 2 == 0) && (x % 2 == 1) {
                unsafe { img.unsafe_get_pixel(x as u32, y as u32).0[0] }
            } else {
                0
            };

            let green = if (y % 2 == 0) && (x % 2 == 0) || (y % 2 == 1) && (x % 2 == 1) {
                unsafe { img.unsafe_get_pixel(x as u32, y as u32).0[0] }
            } else {
                let left = img.ext_index(x - 1, y)[0];
                let right = img.ext_index(x + 1, y)[0];
                let up = img.ext_index(x, y - 1)[0];
                let down = img.ext_index(x, y + 1)[0];

                ((left as u16 + right as u16 + up as u16 + down as u16) / 4) as u8
            };

            *pixel = Rgb([red, green, blue]);
        });

    RgbImage::from_par_fn(width, height, |x, y| {
        let x = x as i32;
        let y = y as i32;
        let (x_mod, y_mod) = (x % 2, y % 2);
        let green = rgb_image.ext_index(x, y)[1];

        let (blue, red) = match (x_mod, y_mod) {
            // origin green only
            (0, 0) | (1, 1) => {
                let left = rgb_image.ext_index(x - 1, y);
                let right = rgb_image.ext_index(x + 1, y);
                let up = rgb_image.ext_index(x, y - 1);
                let down = rgb_image.ext_index(x, y + 1);

                if x_mod == 0 && y_mod == 0 {
                    // orgin top-left green only
                    let blue = ((left[2] as f32 / left[1] as f32
                        + right[2] as f32 / right[1] as f32)
                        / 2.0
                        * green as f32) as u8;
                    let red = ((up[0] as f32 / up[1] as f32 + down[0] as f32 / down[1] as f32)
                        / 2.0
                        * green as f32) as u8;

                    (blue, red)
                } else {
                    // orgin bottom-right green only
                    let blue = ((up[2] as f32 / up[1] as f32 + down[2] as f32 / down[1] as f32)
                        / 2.0
                        * green as f32) as u8;
                    let red = ((left[0] as f32 / left[1] as f32
                        + right[0] as f32 / right[1] as f32)
                        / 2.0
                        * green as f32) as u8;

                    (blue, red)
                }
            }
            (1, 0) | (0, 1) => {
                let up_left = rgb_image.ext_index(x - 1, y - 1);
                let up_right = rgb_image.ext_index(x + 1, y - 1);
                let down_left = rgb_image.ext_index(x - 1, y + 1);
                let down_right = rgb_image.ext_index(x + 1, y + 1);

                if x_mod == 1 {
                    // origin blue only
                    let blue = rgb_image.ext_index(x, y)[2];
                    let red = ((up_left[0] as f32 / up_left[1] as f32
                        + up_right[0] as f32 / up_right[1] as f32
                        + down_left[0] as f32 / down_left[1] as f32
                        + down_right[0] as f32 / down_right[1] as f32)
                        / 4.0
                        * green as f32) as u8;

                    (blue, red)
                } else {
                    // origin red only
                    let red = rgb_image.ext_index(x, y)[0];
                    let blue = ((up_left[2] as f32 / up_left[1] as f32
                        + up_right[2] as f32 / up_right[1] as f32
                        + down_left[2] as f32 / down_left[1] as f32
                        + down_right[2] as f32 / down_right[1] as f32)
                        / 4.0
                        * green as f32) as u8;

                    (blue, red)
                }
            }
            _ => unreachable!(),
        };

        Rgb([red, green, blue])
    })
}

#[cfg(test)]
mod test {
    use image::DynamicImage;

    use super::ExtIndexTrait;

    #[test]
    fn test_index() {
        let img = if let DynamicImage::ImageRgb8(content) =
            image::open("./resources/scan.png").unwrap()
        {
            content
        } else {
            panic!("Cannot read the image correctly.")
        };

        img.ext_index(-1, -1);
        img.ext_index(30, -1);
        img.ext_index(-1, 28);
        img.ext_index(3318, 100);
        img.ext_index(200, 4161);
        img.ext_index(3318, 4161);
        img.ext_index(50, 60);
    }
}
