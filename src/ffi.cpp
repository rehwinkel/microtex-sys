#include <iostream>
#include <latex.h>
#include <cairomm/cairomm.h>
#include <platform/cairo/graphic_cairo.h>

extern "C" void ffi_init_tex()
{
    tex::LaTeX::init();
}

extern "C" void ffi_release_tex()
{
    tex::LaTeX::release();
}

extern "C" void ffi_render_tex(char *code_ptr, size_t code_len, int pixel_width, float text_size, float line_space, char *dest_ptr, size_t dest_len)
{
    std::string code(code_ptr, code_len);
    std::wstring wide_code = tex::utf82wide(code);
    std::cout << code << " " << pixel_width << " " << text_size << " " << line_space << std::endl;
    auto render = tex::LaTeX::parse(wide_code, pixel_width, text_size, line_space, tex::BLACK);
    auto surface = Cairo::ImageSurface::create(Cairo::Format::FORMAT_ARGB32, render->getWidth(), render->getHeight());
    auto cr = Cairo::Context::create(surface);
    tex::Graphics2D_cairo g2(cr);
    render->draw(g2, 0, 0);
    delete render;
    std::string dest(dest_ptr, dest_len);
    surface->write_to_png(dest);
}