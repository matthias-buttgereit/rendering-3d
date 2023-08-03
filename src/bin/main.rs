use rendering_3d::object::Object;

fn main() {
    let width: u32 = 5000;
    let height: u32 = 5000;

    let name = "african_head";

    let model = format!("{name}.obj");
    let texture = format!("{name}_diffuse.tga");

    let mut object = Object::new(&model);
    object.set_texture(&texture);

    object.render_to_image("head.png", width, height).unwrap();

    // let mut img: RgbImage = ImageBuffer::new(width, height);

    // let object_file = std::fs::read_to_string(format!("{filename}.obj"))
    //     .expect("Couldn't read file {file_name}.obj");
    // let object = WavefrontObject::parse_obj_file(&object_file);

    // let texture = image::open(format!("{filename}_diffuse.tga"))
    //     .expect("Couldn't load {filename}.tga as texture.");

    // draw_object(&object, &mut img, &texture);

    // let img = image::imageops::flip_vertical(&img);
    // img.save(format!("{filename}.png"))
    //     .expect("Failed to save Image to File.");
}
