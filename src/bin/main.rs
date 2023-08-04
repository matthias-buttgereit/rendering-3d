use rendering_3d::object::Object;

fn main() {
    let width: u32 = 5000;
    let height: u32 = 5000;

    let name = "diablo3_pose";

    let model = format!("{name}.obj");
    let texture = format!("{name}_diffuse.tga");

    let mut object = Object::new(&model);
    object.set_texture(&texture);

    object.render_to_image("test.png", width, height).unwrap();
}
