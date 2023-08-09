use rendering_3d::object::Object;

fn main() {
    let width: u32 = 600;
    let height: u32 = 600;

    let name = "african_head";

    let model = format!("{name}.obj");
    let texture = format!("{name}_diffuse.tga");

    let origin = (-3.0, 2.0, 3.5);
    let focus = (0.0, 0.0, 0.0);
    let up = (0.0, 1.0, 0.0);

    let mut object = Object::new(&model);
    object.set_texture(&texture);

    object.set_camera(origin, focus, up);

    object
        .render_to_image("african_head.png", width, height)
        .unwrap();
}
