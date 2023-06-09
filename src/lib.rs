use image::RgbImage;

pub fn draw_triangle(image: &mut RgbImage, triangle: &Triangle, color: [u8; 3]) {
    let (x0, y0) = triangle.0;
    let (x1, y1) = triangle.1;
    let (x2, y2) = triangle.2;

    draw_line(image, (x0, y0), (x1, y1), color);
    draw_line(image, (x1, y1), (x2, y2), color);
    draw_line(image, (x2, y2), (x0, y0), color);
}

pub fn draw_line(image: &mut RgbImage, start: (u32, u32), end: (u32, u32), color: [u8; 3]) {
    let (mut x0, mut y0) = start;
    let (mut x1, mut y1) = end;

    let mut steep = false;

    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let (dx, dy) = (x0.abs_diff(x1), y0.abs_diff(y1));
    let derror2 = dy * 2;
    let mut error2 = 0;

    let mut y = y0;

    for x in x0..x1 {
        if steep {
            image.put_pixel(y, x, image::Rgb(color));
        } else {
            image.put_pixel(x, y, image::Rgb(color));
        }
        error2 += derror2;
        if error2 > dx {
            if y1 > y0 {
                y += 1
            } else {
                y -= 1
            };
            error2 -= dx * 2;
        }
    }
}

pub struct Triangle((u32, u32), (u32, u32), (u32, u32));

impl Triangle {
    pub fn new(a: (u32, u32), b: (u32, u32), c: (u32, u32)) -> Self {
        Self(a, b, c)
    }
}
