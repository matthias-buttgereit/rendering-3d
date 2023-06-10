use image::{ImageBuffer, RgbImage};
use rendering_3d::{draw_object, wavefront::WavefrontObject};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1920;

fn main() {
    let file_name = "african_head.obj";

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let object_file = std::fs::read_to_string(file_name).expect("Couldn't read file '{file_name}'");
    let object = WavefrontObject::parse_obj_file(&object_file);

    draw_object(object, &mut img);

    let img = image::imageops::flip_vertical(&img);
    img.save("test.png").expect("Failed to save Image to File.");
}
