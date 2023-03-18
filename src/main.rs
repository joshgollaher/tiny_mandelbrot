use image::{Rgb, RgbImage};

fn main() {
    let mut output = RgbImage::new(3840, 2560);

    for p_x in 0..output.width() {
        for p_y in 0..output.height() {
            let scaled_x = (p_x as f64 / output.width() as f64) * 2.47f64 + -2.0f64;
            let scaled_y = (p_y as f64 / output.height() as f64) * 2.24f64 + -1.12f64;

            let mut x = 0.0f64;
            let mut y = 0.0f64;
            let mut current_iterations = 0;
            const MAX_ITERATIONS: usize = 1000;

            while (x*x + y*y) <= (2*2) as f64 && current_iterations < MAX_ITERATIONS {
                let temp_x = x*x - y*y + scaled_x;
                y = 2f64*x*y + scaled_y;
                x = temp_x;
                current_iterations += 1;
            }

            const COLORS: [[u8; 3]; 16] = [
                [66, 30, 15],
                [25, 7, 26],
                [9, 1, 47],
                [4, 4, 73],
                [0, 7, 100],
                [12, 44, 138],
                [24, 82, 177],
                [57, 125, 209],
                [134, 181, 229],
                [211, 236, 248],
                [241, 233, 191],
                [248, 201, 95],
                [255, 170, 0],
                [204, 128, 0],
                [153, 87, 0],
                [106, 52, 3]
            ];
            let color = if current_iterations == MAX_ITERATIONS { [0, 0, 0] } else { COLORS[current_iterations % 16] };
            output.put_pixel(p_x, p_y, Rgb(color));
        }
    }

    output.save("output.png").expect("Unable to save image!");
}
