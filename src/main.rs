fn main() {
    // Image
    const WIDTH: u32 = 256;//1920;
    const HEIGHT: u32 = 256;//1080;
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);


    // Render
    for j in 0..HEIGHT {
        println!("Lines remaining: {}", j);
        for i in 0..WIDTH {
            let r = i as f64 / (WIDTH - 1) as f64;
            let g = j as f64 / (HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r as f64) as u8;
            let ig = (255.999 * g as f64) as u8;
            let ib = (255.999 * b as f64) as u8;

            // TODO there really has to be a better way than doing heigh minus j minus 1
            imgbuf[(i, HEIGHT - j - 1)] = image::Rgb([ir, ig, ib]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("render.png").unwrap();
}