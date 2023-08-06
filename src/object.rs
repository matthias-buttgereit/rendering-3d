use image::{DynamicImage, ImageBuffer, ImageResult, Pixel, Rgb, RgbImage};
use nalgebra::{matrix, vector, Matrix4, Vector3, Vector4};

use crate::wavefront::WavefrontObject;

// const ZBUFFER_DEPTH: f32 = 255.0;

pub struct Object {
    model: WavefrontObject,
    texture: Option<DynamicImage>,
    camera: Camera,
}

impl Object {
    pub fn new(filename: &str) -> Self {
        let object_file =
            std::fs::read_to_string(filename).expect("Couldn't read file {file_name}.obj");
        let object = WavefrontObject::parse_obj_file(&object_file);

        Self {
            model: object,
            texture: None,
            camera: Camera::new(3.0),
        }
    }

    pub fn set_texture(&mut self, filename: &str) {
        let texture = image::open(filename)
            .unwrap_or_else(|_| panic!("Couldn't load {filename}.tga as texture."));

        let texture = texture.flipv();

        self.texture = Some(texture);
    }

    pub fn render_to_image(&self, name: &str, width: u32, height: u32) -> ImageResult<()> {
        let mut image_buffer: RgbImage = ImageBuffer::new(width, height);

        self.set_pixels_in_buffer(&mut image_buffer);

        let image = image::imageops::flip_vertical(&image_buffer);
        image.save(name)
    }

    fn set_pixels_in_buffer(&self, img: &mut RgbImage) {
        let light_direction = Vector3::new(0.0, 0.0, -1.0);
        let (width, height) = img.dimensions();
        let mut zbuffer: Vec<Vec<u8>> = vec![vec![0; height as usize]; width as usize];

        for face in self.model.faces() {
            let mut triangle_3d = Triangle3d::from_vertices(face.vertices());

            let normal_vector = triangle_3d.get_normal();
            let intensity = light_direction.dot(&normal_vector);

            // let triangle_in_image_space = map_triangle_to_image(&triangle_3d, img);
            self.camera.transform(&mut triangle_3d);

            let color_triangle;

            if let Some(texture) = &self.texture {
                let texture = texture.flipv();
                let texture = texture.as_rgb8().unwrap();
                color_triangle = get_color_triangle(face.texture(), texture);
            } else {
                color_triangle = (Vector3::default(), Vector3::default(), Vector3::default())
            };

            if intensity > 0.0 {
                self.draw_triangle(img, &triangle_3d, intensity, &mut zbuffer, &color_triangle);
            }
        }
    }

    fn draw_triangle(
        &self,
        image: &mut RgbImage,
        triangle: &Triangle3d,
        intensity: f32,
        zbuffer: &mut [Vec<u8>],
        color_triangle: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
    ) {
        let ((x0, y0), (x1, y1)) = get_bounding_box(triangle);
        let x0 = x0.floor().max(0.0) as u32;
        let y0 = y0.floor().max(0.0) as u32;
        let x1 = (x1.ceil() as u32).min(image.dimensions().0 - 1);
        let y1 = (y1.ceil().max(0.0) as u32).min(image.dimensions().1 - 1);

        for x in x0..=x1 {
            for y in y0..=y1 {
                if let Some((s, t, u)) = triangle.barycentric_coords((x, y)) {
                    let z = s * triangle.a.z + t * triangle.b.z + u * triangle.c.z;
                    if zbuffer[x as usize][y as usize] < z as u8 {
                        let color = if let Some(texture) = &self.texture {
                            texture.as_rgb8().unwrap().get_pixel(
                                (color_triangle.0.x * s
                                    + color_triangle.1.x * t
                                    + color_triangle.2.x * u)
                                    as u32,
                                (color_triangle.0.y * s
                                    + color_triangle.1.y * t
                                    + color_triangle.2.y * u)
                                    as u32,
                            )
                        } else {
                            &Rgb([255, 255, 255])
                        };

                        let color = color.map(|x| (x as f32 * intensity) as u8);

                        image.put_pixel(x, y, color);
                        zbuffer[x as usize][y as usize] = z as u8;
                    }
                }
            }
        }
    }
}

// fn map_triangle_to_image(triangle: &Triangle3d, img: &RgbImage) -> Triangle3d {
//     Triangle3d {
//         a: map_3d_point_to_image(triangle.a, img),
//         b: map_3d_point_to_image(triangle.b, img),
//         c: map_3d_point_to_image(triangle.c, img),
//     }
// }

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

// fn map_3d_point_to_image(p: Vector3<f32>, img: &RgbImage) -> Vector3<f32> {
//     let dim = img.dimensions();
//     let (x, y, z) = ((p.x + 1.0) / 2.0, (p.y + 1.0) / 2.0, (p.z + 1.0) / 2.0);
//     let (x, y) = (x * dim.0 as f32, y * dim.1 as f32);
//     let z = z * ZBUFFER_DEPTH;
//     let (x, y) = (
//         x.clamp(0.0, (dim.0 - 1) as f32),
//         y.clamp(0.0, (dim.1 - 1) as f32),
//     );
//     Vector3::new(x, y, z)
// }

fn get_bounding_box(triangle: &Triangle3d) -> ((f32, f32), (f32, f32)) {
    let x_min = triangle.a.x.min(triangle.b.x.min(triangle.c.x));
    let x_max = triangle.a.x.max(triangle.b.x.max(triangle.c.x));

    let y_min = triangle.a.y.min(triangle.b.y.min(triangle.c.y));
    let y_max = triangle.a.y.max(triangle.b.y.max(triangle.c.y));

    ((x_min, y_min), (x_max, y_max))
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

#[derive(Default)]
struct Camera {
    projection: Matrix4<f32>,
    viewport: Matrix4<f32>,
}

impl Camera {
    pub fn new(c: f32) -> Self {
        let projection = matrix![1.0, 0.0, 0.0, 0.0;
                                                                     0.0, 1.0, 0.0, 0.0;
                                                                     0.0, 0.0, 1.0, 0.0;
                                                                     0.0, 0.0, -1.0/c, 1.0];

        let viewport = matrix![960.0, 0.0, 0.0, 960.0;
                                                                   0.0, 960.0, 0.0, 960.0;
                                                                   0.0, 0.0, 127.0, 127.0;
                                                                   0.0, 0.0, 0.0, 1.0];

        Self {
            projection,
            viewport,
        }
    }

    // pub fn set_viewport(&mut self, width: u32, height: u32) {
    //     self.viewport
    // }

    pub fn transform(&self, triangle: &mut Triangle3d) {
        self.transform_vertex(&mut triangle.a);
        self.transform_vertex(&mut triangle.b);
        self.transform_vertex(&mut triangle.c);
    }

    fn transform_vertex(&self, vector: &mut Vector3<f32>) {
        let vector4: Vector4<f32> = vector![vector.x, vector.y, vector.z, 1.0];

        let vector4 = self.viewport * self.projection * vector4;
        let w = vector4.w;
        let vector3 = vector![vector4.x / w, vector4.y / w, vector4.z / w];

        *vector = vector3;
    }
}
