use nalgebra::{matrix, vector, Matrix4, Vector3, Vector4};

use crate::object::Triangle3d;

#[derive(Default)]
pub struct Camera {
    projection: Matrix4<f32>,
    viewport: Matrix4<f32>,
    model_view: Matrix4<f32>,
}

impl Camera {
    pub fn new(c: f32) -> Self {
        let projection = matrix![1.0, 0.0, 0.0, 0.0;
                                0.0, 1.0, 0.0, 0.0;
                                0.0, 0.0, 1.0, 0.0;
                                0.0, 0.0, -1.0/c, 1.0];

        let viewport = matrix![300.0, 0.0, 0.0, 300.0;
                                0.0, 300.0, 0.0, 300.0;
                                0.0, 0.0, 127.0, 127.0;
                                0.0, 0.0, 0.0, 1.0];

        Self {
            projection,
            viewport,
            model_view: Matrix4::identity(),
        }
    }

    pub fn transform(&self, triangle: &mut Triangle3d) {
        self.transform_vertex(&mut triangle.a);
        self.transform_vertex(&mut triangle.b);
        self.transform_vertex(&mut triangle.c);
    }

    fn transform_vertex(&self, vector: &mut Vector3<f32>) {
        let vector4: Vector4<f32> = vector![vector.x, vector.y, vector.z, 1.0];

        let vector4 = self.viewport * self.projection * self.model_view * vector4;
        let w = vector4.w;
        let vector3 = vector![vector4.x / w, vector4.y / w, vector4.z / w];

        *vector = vector3;
    }

    pub fn lookat(&mut self, eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) {
        let z: Vector3<f32> = (eye - center).normalize();
        let x: Vector3<f32> = up.cross(&z).normalize();
        let y: Vector3<f32> = z.cross(&x).normalize();

        let model_view = matrix![x.x, x.y, x.z, -center.x;
                                                               y.x, y.y, y.z, -center.y;
                                                               z.x, z.y, z.z, -center.z;
                                                               0.0, 0.0, 0.0, 1.0];

        self.model_view = model_view;
    }
}
