use libc::c_char;
use libc::size_t;

pub struct LaTeX;

extern "C" {
    fn ffi_render_tex(
        code_ptr: *const c_char,
        code_len: size_t,
        pixel_width: u32,
        text_size: f32,
        line_size: f32,
        fg_color: u32,
        width: *mut u32,
        height: *mut u32,
    ) -> *const libc::c_void;

    fn ffi_render_to_raster(
        raw_render: *const libc::c_void,
        padding: u32,
        image_ptr: *mut u8,
        bg_color: u32,
    );

    fn ffi_init_tex();

    fn ffi_release_tex();
}

/**
 * Pixel data is stored in ARGB format, with byte-order causing the array to contain the
 * pixel data in B-G-R-A order. Swapping bytes 0 and 2 will yield RGBA, which is suitable for exporting to PNG.
 */
pub struct ImageData {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageData {
    pub fn bgra_to_rgba(&mut self) {
        for i in 0..(self.width as usize * self.height as usize) {
            let blue = i * 4;
            let red = i * 4 + 2;
            self.pixels.swap(red, blue);
        }
    }
}

impl LaTeX {
    pub fn init() -> Self {
        unsafe { ffi_init_tex() };
        LaTeX {}
    }

    /**
     * Colors must be supplied in ARGB format.
     */
    pub fn render_latex(
        &self,
        code: &str,
        pixel_width: u32,
        text_size: f32,
        line_size: f32,
        padding: u32,
        fg_color: u32,
        bg_color: u32,
    ) -> Option<ImageData> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let render = unsafe {
            ffi_render_tex(
                code.as_ptr() as *const i8,
                code.len(),
                pixel_width,
                text_size,
                line_size,
                fg_color,
                &mut width,
                &mut height,
            )
        };
        if render.is_null() {
            return None;
        }
        width += padding * 2;
        height += padding * 2;
        let mut image: Vec<u8> = Vec::with_capacity((width * height) as usize * 4);
        unsafe {
            image.set_len((width * height) as usize * 4);
            ffi_render_to_raster(render, padding, image.as_mut_ptr(), bg_color);
        }
        Some(ImageData {
            pixels: image,
            width,
            height,
        })
    }
}

impl Drop for LaTeX {
    fn drop(&mut self) {
        unsafe {
            ffi_release_tex();
        }
    }
}
