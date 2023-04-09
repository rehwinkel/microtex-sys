use libc::c_char;
use libc::size_t;

pub struct LaTeX;

extern "C" {
    fn ffi_render_tex(
        code_ptr: *const c_char,
        code_len: size_t,
        pixel_width: i32,
        text_size: f32,
        line_size: f32,
        dest_ptr: *const c_char,
        dest_len: size_t,
    );

    fn ffi_init_tex();

    fn ffi_release_tex();
}

impl LaTeX {
    pub fn init() -> Self {
        unsafe { ffi_init_tex() };
        LaTeX {}
    }

    pub fn render_to_png(&self, code: &str, dest: &str, text_size: f32, line_size: f32) {
        unsafe {
            ffi_render_tex(
                code.as_ptr() as *const i8,
                code.len(),
                0,
                text_size,
                line_size,
                dest.as_ptr() as *const i8,
                dest.len(),
            );
        }
    }
}

impl Drop for LaTeX {
    fn drop(&mut self) {
        unsafe {
            ffi_release_tex();
        }
    }
}
