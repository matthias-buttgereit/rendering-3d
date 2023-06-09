use image::{ImageBuffer, RgbImage};
use rendering_3d::{draw_triangle, Triangle};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let color = [255u8, 255u8, 255u8];
    let triangle = Triangle::new((1, 1), (255, 255), (1, 255));

    draw_triangle(&mut img, &triangle, color);

    img.save("test.png").expect("Failed to save Image to File.");
}
