use image;
use image::imageops;

use ansi_term::ANSIString;

pub fn image_to_ansi<'a>(img: &'a image::DynamicImage, dst_width: u32, dst_height: u32) -> Vec<u8> {
    let resized = image::DynamicImage::ImageRgba8(
        imageops::resize(
            img,
            dst_width,
            dst_height,
            imageops::FilterType::Nearest
        )
    );

    let resized = resized.to_rgb8();
    resized.as_raw().to_vec()
}
