pub mod wavefront;

use image::RgbImage;
use wavefront::WavefrontObject;

pub fn draw_triangle(image: &mut RgbImage, triangle: &Triangle, color: [u8; 3]) {
    let (x0, y0) = triangle.0;
    let (x1, y1) = triangle.1;
    let (x2, y2) = triangle.2;

    draw_line(image, (x0, y0), (x1, y1), color);
    draw_line(image, (x1, y1), (x2, y2), color);
    draw_line(image, (x2, y2), (x0, y0), color);
}

pub fn draw_line(image: &mut RgbImage, start: (u32, u32), end: (u32, u32), color: [u8; 3]) {
    let (mut x0, mut y0) = start;
    let (mut x1, mut y1) = end;

    let mut steep = false;

    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let (dx, dy) = (x0.abs_diff(x1), y0.abs_diff(y1));
    let derror2 = dy * 2;
    let mut error2: i32 = 0;

    let mut y = y0;

    for x in x0..x1 {
        if steep {
            image.put_pixel(y, x, image::Rgb(color));
        } else {
            image.put_pixel(x, y, image::Rgb(color));
        }

        error2 += derror2 as i32;
        if error2 > dx as i32 {
            if y1 > y0 {
                y += 1
            } else {
                y -= 1
            };
            error2 -= dx as i32 * 2;
        }
    }
}

pub fn map_xy_to_image((x, y): (f32, f32), img: &RgbImage) -> (u32, u32) {
    let (x, y) = ((x + 1.0) / 2.0, (y + 1.0) / 2.0);
    let dim = img.dimensions();
    let (x, y) = (x * dim.0 as f32, y * dim.1 as f32);
    let (x, y) = (
        x.clamp(0.0, (dim.0 - 1) as f32),
        y.clamp(0.0, (dim.1 - 1) as f32),
    );

    (x as u32, y as u32)
}

pub fn draw_object(object: WavefrontObject, img: &mut RgbImage) {
    let color = [255u8, 255u8, 255u8];

    for face in object.faces() {
        let a = face.vertices().0;
        let b = face.vertices().1;
        let c = face.vertices().2;

        let (x0, y0) = map_xy_to_image((a.x, a.y), img);
        let (x1, y1) = map_xy_to_image((b.x, b.y), img);
        let (x2, y2) = map_xy_to_image((c.x, c.y), img);

        let triangle = Triangle::new((x0, y0), (x1, y1), (x2, y2));

        draw_triangle(img, &triangle, color);
    }
}

#[derive(Default, Debug)]
pub struct Triangle((u32, u32), (u32, u32), (u32, u32));

impl Triangle {
    pub fn new(a: (u32, u32), b: (u32, u32), c: (u32, u32)) -> Self {
        Self(a, b, c)
    }
}
