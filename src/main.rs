mod args;
mod errors;
mod image_processor;

use args::Args;
use errors::ImageDataErrors;
use image::GenericImageView;

use image_processor::{combiner, common, floating_image::FloatingImage};

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);

    let (image_1, image_1_format) = common::find_image_form_path(args.image_1);
    let (image_2, image_2_format) = common::find_image_form_path(args.image_2);

    // check format, must be same format
    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = common::standardise_size(image_1, image_2);

    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.out);

    let combined_data = combiner::combine_image(image_1, image_2);

    let _ = output.set_data(combined_data);

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
    )
    .unwrap();

    Ok(())
}
