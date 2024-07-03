use clap::{Arg, Command};
use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    // Define CLI arguments
    let matches = Command::new("Invert Image Colors")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Inverts the colors of an image")
        .arg(
            Arg::new("INPUT")
                .help("Sets the input image file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Sets the output image file")
                .required(true)
                .index(2),
        )
        .get_matches();

    // Get the input and output file paths
    let input_path = matches.get_one::<String>("INPUT").expect("INPUT is required");
    let output_path = matches.get_one::<String>("OUTPUT").expect("OUTPUT is required");

    // Load the image from the input file
    let img = image::open(input_path).expect("Failed to open input image");

    // Create a new image buffer with the same dimensions
    let mut img_inverted = ImageBuffer::new(img.width(), img.height());

    // Iterate over each pixel and invert its colors
    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;
        let inverted_pixel = Rgba([255 - data[0], 255 - data[1], 255 - data[2], data[3]]);
        img_inverted.put_pixel(x, y, inverted_pixel);
    }

    // Save the inverted image to the output file
    img_inverted.save(output_path).expect("Failed to save output image");
}
