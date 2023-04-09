use microtex_sys::LaTeX;

fn main() {
    let tex = LaTeX::init();
    tex.render_to_png("\\sum_{n=0}^{3} n", "amongus.png", 20.0, 10.0);
}
