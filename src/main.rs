use clap::{Arg, App};
use std::io;

const ASCII_VALUES: &[u8] = "@%#*+=-:. ".as_bytes();

fn image_average(tile: image::GrayImage) -> u32 {
    let pixels = tile.pixels();
    let mut total: u128 = 0;
    let pixel_count = pixels.count() as u128;
    for pixel in tile.pixels() {
        total += pixel.0[0] as u128;
    }
    (total / pixel_count) as u32
}

fn convert(img: image::DynamicImage, columns: u32, scale: f32) -> Vec<String> {
    let mut grayscale = img.to_luma8();
    let width = grayscale.dimensions().0;
    let height = grayscale.dimensions().1;
    let tile_width = width / columns;
    let tile_height = tile_width as f32 / scale;

    let rows = (height as f32 / tile_height) as u32;
    let mut ascii_image: Vec<String> = Vec::new();

    for i in 0..rows {
        let y = i * tile_height as u32;
        let mut y_offset = tile_height as u32;

        if i == rows - 1 {
            y_offset = height - y;
        }

        ascii_image.push(String::from(""));

        for j in 0..columns {
            let x: u32 = j * tile_width;
            let mut x_offset = tile_width;

            if j == columns - 1 {
                x_offset = width - x;
            }
            let cropped_section = image::imageops::crop(&mut grayscale, x, y, x_offset, y_offset);
            let avg_pixel = image_average(cropped_section.to_image());
            let idx = ((avg_pixel * 9) / 255) as usize;
            let ascii_val = ASCII_VALUES[idx] as char;
            ascii_image[i as usize].push(ascii_val);
        }

    }
    ascii_image
}


fn main() -> io::Result<()> {
    let matches = App::new("ASCII Art Generator")
        .version("0.0.1")
        .author("Albert Chung")
        .about("Convert images into ascii art.")
        .arg(Arg::with_name("image-path")
            .short("i")
            .long("image-path")
            .takes_value(true)
            .help("Path to the image"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Path to the output file"))
        .arg(Arg::with_name("scale")
            .short("s")
            .long("scale")
            .default_value("0.43")
            .takes_value(true)
            .help("Image scale"))
        .arg(Arg::with_name("columns")
            .short("c")
            .long("columns")
            .takes_value(true)
            .default_value("70")
            .help("Number of columns in ascii image"))
        .get_matches();

    let image_path = matches.value_of("image-path").unwrap();
    let output = matches.value_of("output").unwrap_or("output.txt");
    let scale: f32 = matches.value_of("scale").unwrap().parse().unwrap();
    let columns: u32 = matches.value_of("columns").unwrap().parse().unwrap();

    let img = image::open(image_path).unwrap();
    let ascii_image = convert(img, columns, scale);

    std::fs::write(output, ascii_image.join("\n"))?;
    println!("{}", ascii_image.join("\n"));
    Ok(())
}

