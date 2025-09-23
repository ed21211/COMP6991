use bmp::{Image, Pixel};

fn main() {
    let path = std::env::args().nth(1).expect("provide a file");
    let operation = std::env::args().nth(2).expect("provide an operation");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "finland" {
        draw_finland(path.as_str());
        // Add more cases here!
    } else {
        eprintln!("The operation {operation} was not recognised!");
    }
}
fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));    
    image.save(path).expect("This should save correctly.");
}


fn draw_finland(path: &str) {
    let mut image = Image::new(150, 100);
    
    for x in 0..150 {
        for y in 0..100 {
            if (x > 20 && x < 40) || (y > 40 && y < 60) {
                image.set_pixel(x, y, Pixel::new(255,255,255));
            } else {
                image.set_pixel(x, y, Pixel::new(0,0, 255));
            }
        }
    }
    
    image.save(path).expect("This should save correctly.");
    // println!("{path}");
    let _ = image.save("img.bmp").unwrap_or_else(|e| {
        panic!("Failed to save: {}", e)
    });
}
