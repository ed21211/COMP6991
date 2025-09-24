use bmp::{Image, Pixel};
use num_integer::{Roots};

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let mut picture = Image::new(201, 201);

    let center = Point{x:101, y:101};
    let radius = 90;
    let mut r = 0.0;
    let mut g = 0.0;
    for row in 0..=200 {
        for col in 0..=200 {
            let d = distance(Point{x: row as i32, y: col as i32}, &center);
            if d < radius {
                // let b = 255 - ((d as f32 / radius as f32) * 255.0) as u8;
                picture.set_pixel(row, col, Pixel::new(r as u8, g as u8, 128));
            } else {
                    picture.set_pixel(row, col, Pixel::new(255, 255, 255));
            }
            g = g + (255.0 / 201.0);
        }
        g = 0.0;
        r = r + (255.0 / 201.0);
    }
    let _ = picture.save("img.bmp");
}

fn distance(a: Point, b: &Point) -> u8 {
    let dx = (a.x - b.x).pow(2);
    let dy = (a.y - b.y).pow(2);
    let dist = (dy + dx).sqrt() as u8;

    return dist;
}
