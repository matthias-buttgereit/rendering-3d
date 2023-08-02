use image::{ImageBuffer, RgbImage};
use rendering_3d::{draw_object, wavefront::WavefrontObject};

fn main() {
    let width: u32 = 1920;
    let height: u32 = 1920;

    let filename = "african_head";

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let object_file = std::fs::read_to_string(format!("{filename}.obj"))
        .expect("Couldn't read file {file_name}.obj");
    let object = WavefrontObject::parse_obj_file(&object_file);

    let texture = image::open(format!("{filename}_diffuse.tga"))
        .expect("Couldn't load {filename}.tga as texture.");

    draw_object(object, &mut img, &texture);

    let img = image::imageops::flip_vertical(&img);
    img.save("test.png").expect("Failed to save Image to File.");
}
