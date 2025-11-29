use core::panic;
use std::{cell::LazyCell, cmp, collections::HashMap, io};

use clap::{Parser, Subcommand};
use image::{ColorType, DynamicImage, ImageReader, Pixel, RgbaImage};
use owo_colors::OwoColorize;
use regex::{Regex, RegexBuilder};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// The source image file path
    #[arg(global = true)]
    target: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Goes through the colors in an image and changes them to a specified color
    Recolor,
    /// Deconstrucst the image into a json of its pixels
    Deconstruct,
    /// Reconstructs a image from a json of its pixels
    Reconstruct,
}

fn main() {
    let cli_args = Cli::parse();
    let target = cli_args
        .target
        .as_deref()
        .expect("No image file specified!");
    let source_image = ImageReader::open(target)
        .expect("Failed to open image file")
        .decode()
        .expect("Failed to decode image");

    match cli_args.command {
        Command::Recolor => {
            println!("Recoloring image at path: {}", target);
            recolor(source_image);
        }
        Command::Deconstruct => {
            println!("Deconstructing image at path: {}", target);
            // Add deconstruct logic here
        }
        Command::Reconstruct => {
            println!("Reconstructing image from data at path: {}", target);
            // Add reconstruct logic here
        }
    }
}

fn recolor(mut source_image: DynamicImage) {
    println!("Recolor function called.");
    match source_image.color() {
        ColorType::L8 => println!("Image is encoded as: L8"),
        ColorType::La8 => println!("Image is encoded as: La8"),
        ColorType::Rgb8 => println!("Image is encoded as: Rgb8"),
        ColorType::Rgba8 => recolor_rgba8(
            source_image
                .as_mut_rgba8()
                .expect("Failed to get RGBA8 image buffer"),
        ),
        ColorType::L16 => println!("Image is encoded as: L16"),
        ColorType::La16 => println!("Image is encoded as: La16"),
        ColorType::Rgb16 => println!("Image is encoded as: Rgb16"),
        ColorType::Rgba16 => println!("Image is encoded as: Rgba16"),
        ColorType::Rgb32F => println!("Image is encoded as: Rgb32F"),
        ColorType::Rgba32F => println!("Image is encoded as: Rgba32F"),
        _ => panic!("Unsupported color type for recoloring"),
    }
}

const HEX_COLOR_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    RegexBuilder::new(r"#?([0-9A-F]{2})([0-9A-F]{2})([0-9A-F]{2})")
        .case_insensitive(true)
        .build()
        .expect("Failed to compile regex")
});

fn recolor_rgba8(source_image: &mut RgbaImage) {
    let mut pixel_map: HashMap<[u8; 4], Vec<(u32, u32)>> = HashMap::new();
    for (x, y, rgba_val) in source_image.enumerate_pixels() {
        // We only care about non-transparent pixels
        if rgba_val.alpha() != 0 {
            pixel_map
                .entry(rgba_val.0)
                .and_modify(|locations| {
                    locations.push((x, y));
                })
                .or_insert_with(|| vec![(x, y)]);
        }
    }
    let mut input = String::new();
    for (color, locations) in pixel_map {
        let color_swatch = "    ".on_truecolor(color[0], color[1], color[2]);
        let cutoff = cmp::min(10, locations.len());
        println!(
            "Color: #{:02x}{:02x}{:02x} {} at locations: {:?}...",
            color[0],
            color[1],
            color[2],
            color_swatch,
            &locations[..cutoff]
        );
        println!(
            "Enter new color for this color (in hex format, e.g. #ff00ff), or press Enter to keep the same color:"
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let new_color = HEX_COLOR_REGEX.captures(input.trim());
        if let Some(caps) = new_color {
            let r = u8::from_str_radix(&caps[1], 16).expect("Failed to parse red component");
            let g = u8::from_str_radix(&caps[2], 16).expect("Failed to parse green component");
            let b = u8::from_str_radix(&caps[3], 16).expect("Failed to parse blue component");
            for (x, y) in locations {
                source_image.put_pixel(x, y, image::Rgba([r, g, b, color[3]]));
            }
        }
        input.clear();
    }
    println!("Enter a filename for the recolored image: (default: recolor_output.png)");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.len() == 0 {
        input = "recolor_output.png".into();
    }
    source_image
        .save(input.trim())
        .expect("Failed to save recolored image");
}
