use image::{ImageBuffer, ImageResult, Pixel, Rgb, RgbImage};
use nalgebra::Vector3;

use crate::{camera::Camera, wavefront::WavefrontObject};

pub struct Object {
    model: WavefrontObject,
    texture: RgbImage,
    camera: Camera,
}

impl Object {
    pub fn new(filename: &str) -> Self {
        let object_file =
            std::fs::read_to_string(filename).expect("Couldn't read file {file_name}.obj");
        let object = WavefrontObject::parse_obj_file(&object_file);

        Self {
            model: object,
            texture: RgbImage::default(),
            camera: Camera::new(5.0),
        }
    }

    pub fn set_camera(
        &mut self,
        origin: (f32, f32, f32),
        focus: (f32, f32, f32),
        up: (f32, f32, f32),
    ) {
        let origin: Vector3<f32> = Vector3::new(origin.0, origin.1, origin.2);
        let focus: Vector3<f32> = Vector3::new(focus.0, focus.1, focus.2);
        let up: Vector3<f32> = Vector3::new(up.0, up.1, up.2);

        let f = (origin - focus).norm();

        self.camera.set_projection(f);
        self.camera.lookat(origin, focus, up);
    }

    pub fn set_texture(&mut self, filename: &str) {
        let texture = image::open(filename)
            .unwrap_or_else(|_| panic!("Couldn't load {filename}.tga as texture."));

        let texture = texture.flipv();
        let rgb = texture.to_rgb8();
        self.texture = rgb;
    }

    pub fn render_to_image(&mut self, name: &str, width: u32, height: u32) -> ImageResult<()> {
        let mut image_buffer: RgbImage = ImageBuffer::new(width, height);

        self.camera
            .set_viewport(0.0, 0.0, width as f32, height as f32);

        self.set_pixels_in_buffer(&mut image_buffer);

        let image = image::imageops::flip_vertical(&image_buffer);
        image.save(name)
    }

    fn set_pixels_in_buffer(&self, img: &mut RgbImage) {
        let light_direction = Vector3::new(-2.0, -1.0, -1.0).normalize();

        let (width, height) = img.dimensions();
        let mut zbuffer: Vec<Vec<f32>> = vec![vec![0.0; height as usize]; width as usize];

        for face in self.model.faces() {
            let average = (face.normals().0 + face.normals().1 + face.normals().2) / 3.0;
            if average.dot(self.camera.view_dir()) < 0.0 {
                continue;
            }
            let mut triangle_3d = Triangle3d::from_vertices(face.vertices());
            self.camera.transform(&mut triangle_3d);

            // let normal_vector = triangle_3d.get_normal();
            let color_triangle = get_color_triangle(face.texture(), &self.texture);

            self.draw_triangle(
                img,
                &triangle_3d,
                &light_direction,
                &mut zbuffer,
                &color_triangle,
                face.normals(),
            );
        }

        draw_grey_image(zbuffer, width, height);
    }

    fn draw_triangle(
        &self,
        image: &mut RgbImage,
        triangle: &Triangle3d,
        intensity: &Vector3<f32>,
        zbuffer: &mut [Vec<f32>],
        texture: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
        normals: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
    ) {
        let ((x0, y0), (x1, y1)) = get_bounding_box(triangle);
        let (x0, y0, x1, y1) = clamp(x0, y0, x1, y1, image.dimensions());

        for x in x0..=x1 {
            for y in y0..=y1 {
                if let Some((s, t, u)) = triangle.contains_point((x, y)) {
                    let z = s * triangle.a.z + t * triangle.b.z + u * triangle.c.z;
                    let normal = (s * normals.0 + t * normals.1 + u * normals.2).normalize();
                    self.draw_point(
                        (s, t, u),
                        zbuffer,
                        (x, y, z),
                        texture,
                        intensity,
                        image,
                        normal,
                    );
                }
            }
        }
    }

    fn draw_point(
        &self,
        (s, t, u): (f32, f32, f32),
        zbuffer: &mut [Vec<f32>],
        (x, y, z): (u32, u32, f32),
        texture: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
        intensity: &Vector3<f32>,
        image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        normal: Vector3<f32>,
    ) {
        if zbuffer[x as usize][y as usize] < z {
            let texture_x = (texture.0.x * s + texture.1.x * t + texture.2.x * u) as u32;
            let texture_y = (texture.0.y * s + texture.1.y * t + texture.2.y * u) as u32;
            let intensity = (-1.0) * intensity.dot(&normal);

            let color = self
                .texture
                .get_pixel(texture_x, texture_y)
                .map(|x| (x as f32 * intensity) as u8);

            // let color: Rgb<u8> =
            //     Rgb([255, 255, 255]).map(|x| (x as f32 * (-1.0) * intensity.dot(&normal)) as u8);

            image.put_pixel(x, y, color);
            zbuffer[x as usize][y as usize] = z;
        }
    }
}

fn clamp(x0: f32, y0: f32, x1: f32, y1: f32, (width, height): (u32, u32)) -> (u32, u32, u32, u32) {
    let x0 = x0.floor().max(0.0) as u32;
    let y0 = y0.floor().max(0.0) as u32;
    let x1 = (x1.ceil() as u32).min(width - 1);
    let y1 = (y1.ceil().max(0.0) as u32).min(height - 1);
    (x0, y0, x1, y1)
}

fn draw_grey_image(zbuffer: Vec<Vec<f32>>, width: u32, height: u32) {
    let grey: Vec<Vec<u8>> = zbuffer
        .iter()
        .map(|x| x.iter().map(|y| *y as u8).collect())
        .collect();

    let mut grey_image: RgbImage = ImageBuffer::new(width, height);
    for (x, line) in grey.iter().enumerate() {
        for (y, color) in line.iter().enumerate() {
            let color = Rgb([*color, *color, *color]);
            grey_image.put_pixel(x as u32, (width - 1) - y as u32, color);
        }
    }
    grey_image.save("grey.png").unwrap();
}

fn get_color_triangle(
    vertices: &(Vector3<f32>, Vector3<f32>, Vector3<f32>),
    texture: &RgbImage,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let (height, width) = (texture.height() as f32, texture.width() as f32);

    let a = Vector3::new(vertices.0.x * width, vertices.0.y * height, 0.0);
    let b = Vector3::new(vertices.1.x * width, vertices.1.y * height, 0.0);
    let c = Vector3::new(vertices.2.x * width, vertices.2.y * height, 0.0);

    (a, b, c)
}

fn get_bounding_box(triangle: &Triangle3d) -> ((f32, f32), (f32, f32)) {
    let x_min = triangle.a.x.min(triangle.b.x.min(triangle.c.x));
    let x_max = triangle.a.x.max(triangle.b.x.max(triangle.c.x));

    let y_min = triangle.a.y.min(triangle.b.y.min(triangle.c.y));
    let y_max = triangle.a.y.max(triangle.b.y.max(triangle.c.y));

    ((x_min, y_min), (x_max, y_max))
}

pub struct Triangle3d {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub c: Vector3<f32>,
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

    pub fn contains_point(&self, (x, y): (u32, u32)) -> Option<(f32, f32, f32)> {
        let x = x as f32 + 0.5;
        let y = y as f32 + 0.5;

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
