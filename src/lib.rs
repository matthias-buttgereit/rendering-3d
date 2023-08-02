pub mod wavefront;

use image::{DynamicImage, Rgb, RgbImage};
use nalgebra::Vector3;
use wavefront::WavefrontObject;

const ZBUFFER_DEPTH: f32 = 255.0;

fn draw_triangle(
    image: &mut RgbImage,
    triangle: &Triangle3d,
    intensity: f32,
    zbuffer: &mut [Vec<u8>],
    texture: &RgbImage,
    texture_vertices: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
) {
    let ((x0, y0), (x1, y1)) = get_bounding_box(triangle);
    let x0 = x0.floor() as u32;
    let y0 = y0.floor() as u32;
    let x1 = x1.ceil() as u32;
    let y1 = y1.ceil() as u32;
    let color_triangle = get_color_triangle(texture_vertices, texture);

    for x in x0..=x1 {
        for y in y0..=y1 {
            if let Some((s, t, u)) = triangle.barycentric_coords((x, y)) {
                let z = s * triangle.a.z + t * triangle.b.z + u * triangle.c.z;

                let color = texture.get_pixel(
                    (color_triangle.0.x * s + color_triangle.1.x * t + color_triangle.2.x * u)
                        as u32,
                    (color_triangle.0.y * s + color_triangle.1.y * t + color_triangle.2.y * u)
                        as u32,
                );

                let color = [
                    (color.0[0] as f32 * intensity) as u8,
                    (color.0[1] as f32 * intensity) as u8,
                    (color.0[2] as f32 * intensity) as u8,
                ];

                if zbuffer[x as usize][y as usize] < z as u8 {
                    image.put_pixel(x, y, Rgb(color));
                    zbuffer[x as usize][y as usize] = z as u8;
                }
            }
        }
    }
}

fn get_color_triangle(
    texture_vertices: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
    texture: &image::ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let (height, width) = (texture.height() as f32, texture.width() as f32);
    let a = Vector3::new(
        texture_vertices.0.x * width,
        texture_vertices.0.y * height,
        0.0,
    );

    let b = Vector3::new(
        texture_vertices.1.x * width,
        texture_vertices.1.y * height,
        0.0,
    );

    let c = Vector3::new(
        texture_vertices.2.x * width,
        texture_vertices.2.y * height,
        0.0,
    );

    (a, b, c)
}

fn get_bounding_box(triangle: &Triangle3d) -> ((f32, f32), (f32, f32)) {
    let x_min = triangle.a.x.min(triangle.b.x.min(triangle.c.x));
    let x_max = triangle.a.x.max(triangle.b.x.max(triangle.c.x));

    let y_min = triangle.a.y.min(triangle.b.y.min(triangle.c.y));
    let y_max = triangle.a.y.max(triangle.b.y.max(triangle.c.y));

    ((x_min, y_min), (x_max, y_max))
}

fn map_3d_point_to_image(p: Vector3<f32>, img: &RgbImage) -> Vector3<f32> {
    let dim = img.dimensions();
    let (x, y, z) = ((p.x + 1.0) / 2.0, (p.y + 1.0) / 2.0, (p.z + 1.0) / 2.0);

    let (x, y) = (x * dim.0 as f32, y * dim.1 as f32);

    let z = z * ZBUFFER_DEPTH;

    let (x, y) = (
        x.clamp(0.0, (dim.0 - 1) as f32),
        y.clamp(0.0, (dim.1 - 1) as f32),
    );

    Vector3::new(x, y, z)
}

pub fn draw_object(object: WavefrontObject, img: &mut RgbImage, texture_map: &DynamicImage) {
    let light_direction = Vector3::new(0.0, 0.0, -1.0);
    let (w, h) = img.dimensions();
    let mut zbuffer: Vec<Vec<u8>> = vec![vec![0; h as usize]; w as usize];

    let texture = texture_map.flipv();
    let texture = texture.as_rgb8().unwrap();

    for face in object.faces() {
        let triangle3d = Triangle3d::from_vertices(face.vertices());
        let normal_vector = triangle3d.get_normal();

        let intensity = light_direction.dot(&normal_vector);

        let drawable_triangle = map_triangle_to_image(&triangle3d, img);

        if intensity > 0.0 {
            draw_triangle(
                img,
                &drawable_triangle,
                intensity,
                &mut zbuffer,
                texture,
                face.texture(),
            );
        }
    }
}

fn map_triangle_to_image(triangle: &Triangle3d, img: &RgbImage) -> Triangle3d {
    Triangle3d {
        a: map_3d_point_to_image(triangle.a, img),
        b: map_3d_point_to_image(triangle.b, img),
        c: map_3d_point_to_image(triangle.c, img),
    }
}

struct Triangle3d {
    a: Vector3<f32>,
    b: Vector3<f32>,
    c: Vector3<f32>,
}

impl Triangle3d {
    pub fn from_vertices(v: &(Vector3<f32>, Vector3<f32>, Vector3<f32>)) -> Self {
        Self {
            a: v.0,
            b: v.1,
            c: v.2,
        }
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

    pub fn get_vertices(&self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        (self.a, self.b, self.c)
    }

    pub fn barycentric_coords(&self, (x, y): (u32, u32)) -> Option<(f32, f32, f32)> {
        let x = x as f32;
        let y = y as f32;

        let (p1, p2, p3) = self.get_vertices();

        let s = ((p2.y - p3.y) * (x - p3.x) + (p3.x - p2.x) * (y - p3.y))
            / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));

        let t = ((p3.y - p1.y) * (x - p3.x) + (p1.x - p3.x) * (y - p3.y))
            / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));

        if s >= 0.0 && t >= 0.0 && s + t <= 1.0 {
            let u = 1.0 - s - t;
            Some((s, t, u))
        } else {
            None
        }
    }
}
