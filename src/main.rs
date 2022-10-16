use pixel_canvas::{Canvas, Color};

fn main() {
    // RayTrace
}

fn drawRGB(mut rgb_val: Vec<[u8; 3]>, height: usize, width: usize) {
    let canvas = Canvas::new(width, height)
        .title("Tile");

    canvas.render(|_, image| {
        let width = image.width() as usize;
        for row in image.chunks_mut(width) {
            for pixel in row.iter_mut() {
                let r = rgb_val[0][0];
                let g = rgb_val[0][1];
                let b = rgb_val[0][2];
                rgb_val.pop();
                *pixel = Color::rgb(r, g, b);
            }
        }
    });
}