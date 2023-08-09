use rendering_3d::object::Object;

fn main() {
    let width: u32 = 600;
    let height: u32 = 600;

    let name = "african_head";

    let model = format!("{name}.obj");
    let texture = format!("{name}_diffuse.tga");

    let mut object = Object::new(&model);
    object.set_texture(&texture);

    object
        .render_to_image("african_head.png", width, height)
        .unwrap();
}
