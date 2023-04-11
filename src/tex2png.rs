use std::{fs::File, io::BufWriter, path::Path};

use microtex_sys::LaTeX;

fn main() {
    let tex = LaTeX::init();
    let mut image_data = tex
        .render_latex(
            "\\begin{cmatrix} 1&2 \\\\ 3&4 \\end{cmatrix}",
            0,
            50.0,
            10.0,
            10,
            0xFFFF0000,
            0xFF0000FF,
        )
        .unwrap();
    image_data.bgra_to_rgba();
    let path = Path::new("test_image.png");
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, image_data.width, image_data.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut header = encoder.write_header().unwrap();
    header.write_image_data(&image_data.pixels).unwrap();
}
