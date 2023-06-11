pub mod wavefront;

use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use wavefront::WavefrontObject;

fn draw_triangle(image: &mut RgbImage, triangle: &Triangle2d<u32>, color: [u8; 3]) {
    let ((x0, y0), (x1, y1)) = get_bounding_box(triangle);

    for x in x0..=x1 {
        for y in y0..=y1 {
            if triangle.contains((x, y)) {
                image.put_pixel(x, y, Rgb(color));
            }
        }
    }
}

fn get_bounding_box(triangle: &Triangle2d<u32>) -> ((u32, u32), (u32, u32)) {
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
    let light_direction = Vector3::new(0.0, 0.0, -1.0);
    let (w, h) = img.dimensions();
    let mut _zbuffer: Vec<Vec<i32>> = vec![vec![i32::MIN; h as usize]; w as usize];

    for face in object.faces() {
        let triangle3d = Triangle3d::new(face.vertices().0, face.vertices().1, face.vertices().2);
        let normal_vector = triangle3d.get_normal();

        let intensity = light_direction.dot(&normal_vector);

        let a = face.vertices().0;
        let b = face.vertices().1;
        let c = face.vertices().2;

        let (x0, y0) = map_xy_to_image((a.x, a.y), img);
        let (x1, y1) = map_xy_to_image((b.x, b.y), img);
        let (x2, y2) = map_xy_to_image((c.x, c.y), img);

        let triangle = Triangle2d::new((x0, y0), (x1, y1), (x2, y2));

        if intensity > 0.0 {
            draw_triangle(
                img,
                &triangle,
                [
                    (255.0 * intensity) as u8,
                    (255.0 * intensity) as u8,
                    (255.0 * intensity) as u8,
                ],
            );
        }
    }
}

#[derive(Default, Debug)]
pub struct Triangle2d<N> {
    a: (N, N),
    b: (N, N),
    c: (N, N),
}

pub struct Triangle3d<N> {
    a: Vector3<N>,
    b: Vector3<N>,
    c: Vector3<N>,
}

impl Triangle3d<f32> {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>) -> Self {
        Self { a, b, c }
    }

    pub fn get_normal(&self) -> Vector3<f32> {
        let v1 = self.b - self.a;
        let v2 = self.c - self.a;

        Vector3::new(
            (v1.z * v2.y) - (v1.y * v2.z),
            (v1.x * v2.z) - (v1.z * v2.x),
            (v1.y * v2.x) - (v1.x * v2.y),
        )
        .normalize()
    }
}

impl Triangle2d<u32> {
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
