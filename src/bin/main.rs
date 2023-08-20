use rendering_3d::object::Object;

fn main() {
    let width: u32 = 600;
    let height: u32 = 600;

    let name = "african_head";

    let model = format!("data/{name}.obj");
    let texture = format!("data/{name}_diffuse.tga");
    let output = format!("output/{name}.png");

    let origin = (0.0, 0.0, -3.0);
    let focus = (0.0, 0.0, 0.0);
    let up = (0.0, 1.0, 0.0);

    let mut object = Object::new(&model);
    object.set_texture(&texture);

    object.set_camera(origin, focus, up);

    object.render_to_image(&output, width, height).unwrap();
}
