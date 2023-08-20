use nalgebra::{matrix, vector, Matrix4, Vector3, Vector4};

use crate::object::Triangle3d;

#[derive(Default)]
pub struct Camera {
    projection: Matrix4<f32>,
    viewport: Matrix4<f32>,
    model_view: Matrix4<f32>,
    viewing_direction: Vector3<f32>,
}

impl Camera {
    pub fn new(c: f32) -> Self {
        let mut projection: Matrix4<f32> = Matrix4::identity();
        projection.m43 = -1.0 / c;

        let viewport: Matrix4<f32> = matrix![300.0, 0.0, 0.0, 300.0; 0.0, 300.0, 0.0, 300.0;  0.0, 0.0, 127.0, 127.0; 0.0, 0.0, 0.0, 1.0];

        Self {
            projection,
            viewport,
            model_view: Matrix4::identity(),
            viewing_direction: Vector3::default(),
        }
    }

    pub fn transform(&self, triangle: &mut Triangle3d) {
        self.transform_vertex(&mut triangle.a);
        self.transform_vertex(&mut triangle.b);
        self.transform_vertex(&mut triangle.c);
    }

    pub fn transform_vertex(&self, vector: &mut Vector3<f32>) {
        let vector4: Vector4<f32> = vector![vector.x, vector.y, vector.z, 1.0];

        let vector4 = self.viewport * self.projection * self.model_view * vector4;
        let w = vector4.w;
        let vector3 = vector![vector4.x / w, vector4.y / w, vector4.z / w];

        *vector = vector3;
    }

    pub fn set_viewport(&mut self, start_x: f32, start_y: f32, width: f32, height: f32) {
        self.viewport = matrix![width/2.0, 0.0, 0.0, start_x+width/2.0; 0.0, height/2.0, 0.0, start_y+height/2.0; 0.0, 0.0, 127.0, 127.0; 0.0, 0.0, 0.0, 1.0]
    }

    pub fn set_projection(&mut self, f: f32) {
        let mut projection = Matrix4::identity();
        projection.m43 = -1.0 / f;
        self.projection = projection;
    }

    pub fn lookat(&mut self, eye: Vector3<f32>, focus: Vector3<f32>, up: Vector3<f32>) {
        self.viewing_direction = (focus - eye).normalize();
        let z: Vector3<f32> = self.viewing_direction;
        let x: Vector3<f32> = up.cross(&z).normalize();
        let y: Vector3<f32> = z.cross(&x).normalize();

        self.model_view = matrix![x.x, x.y, x.z, -focus.x; y.x, y.y, y.z, -focus.y; z.x, z.y, z.z, -focus.z; 0.0, 0.0, 0.0, 1.0];
    }

    pub fn view_dir(&self) -> Vector3<f32> {
        self.viewing_direction
    }
}
