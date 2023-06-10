pub mod wavefront;

use image::{Rgb, RgbImage};
use wavefront::WavefrontObject;

fn draw_triangle(image: &mut RgbImage, triangle: &Triangle, color: [u8; 3]) {
    let ((x0, y0), (x1, y1)) = get_bounding_box(triangle);

    for x in x0..=x1 {
        for y in y0..=y1 {
            if triangle.contains((x, y)) {
                image.put_pixel(x, y, Rgb(color));
            }
        }
    }
}

fn get_bounding_box(triangle: &Triangle) -> ((u32, u32), (u32, u32)) {
    let x_min = triangle.a.0.min(triangle.b.0.min(triangle.c.0));
    let x_max = triangle.a.0.max(triangle.b.0.max(triangle.c.0));

    let y_min = triangle.a.1.min(triangle.b.1.min(triangle.c.1));
    let y_max = triangle.a.1.max(triangle.b.1.max(triangle.c.1));

    ((x_min, y_min), (x_max, y_max))
}

fn map_xy_to_image((x, y): (f32, f32), img: &RgbImage) -> (u32, u32) {
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
pub struct Triangle {
    a: (u32, u32),
    b: (u32, u32),
    c: (u32, u32),
}

impl Triangle {
    pub fn new(a: (u32, u32), b: (u32, u32), c: (u32, u32)) -> Self {
        Self { a, b, c }
    }

    pub fn contains(&self, (x, y): (u32, u32)) -> bool {
        let p1 = (self.a.0 as f32, self.a.1 as f32);
        let p2 = (self.b.0 as f32, self.b.1 as f32);
        let p3 = (self.c.0 as f32, self.c.1 as f32);
        let x = x as f32;
        let y = y as f32;

        let area_triangle =
            0.5 * (-p2.1 * p3.0 + p1.1 * (p3.0 - p2.0) + p1.0 * (p2.1 - p3.1) + p2.0 * p3.1);

        let s = 1.0 / (2.0 * area_triangle)
            * (p1.1 * p3.0 - p1.0 * p3.1 + (p3.1 - p1.1) * x + (p1.0 - p3.0) * y);
        let t = 1.0 / (2.0 * area_triangle)
            * (p1.0 * p2.1 - p1.1 * p2.0 + (p1.1 - p2.1) * x + (p2.0 - p1.0) * y);

        s >= 0.0 && t >= 0.0 && s + t <= 1.0
    }
}
