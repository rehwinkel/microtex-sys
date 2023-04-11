#include <iostream>
#include <cstring>
#include <latex.h>
#include <pangomm/init.h>
#include <cairomm/cairomm.h>
#include <platform/cairo/graphic_cairo.h>

extern "C" void ffi_init_tex()
{
    Pango::init();
    tex::LaTeX::init();
}

extern "C" void ffi_release_tex()
{
    tex::LaTeX::release();
}

extern "C" void *ffi_render_tex(char *code_ptr, size_t code_len, uint32_t pixel_width, float text_size, float line_space, uint32_t fg_color, uint32_t *width, uint32_t *height)
{
    std::string code(code_ptr, code_len);
    std::wstring wide_code = tex::utf82wide(code);
    try
    {
        auto render = tex::LaTeX::parse(wide_code, pixel_width, text_size, line_space, fg_color);
        *width = render->getWidth();
        *height = render->getHeight();
        return render;
    }
    catch (std::exception &e)
    {
        return nullptr;
    }
}

extern "C" void ffi_render_to_raster(void *raw_render, uint32_t padding, uint8_t *image_ptr, uint32_t bg_color)
{
    tex::TeXRender *render = static_cast<tex::TeXRender *>(raw_render);
    int width = render->getWidth() + padding * 2;
    int height = render->getHeight() + padding * 2;
    auto surface = Cairo::ImageSurface::create(Cairo::Format::FORMAT_ARGB32, width, height);
    auto cr = Cairo::Context::create(surface);
    uint8_t alpha = bg_color >> 24;
    uint8_t red = bg_color >> 16;
    uint8_t green = bg_color >> 8;
    uint8_t blue = bg_color;
    cr->set_source_rgba(
        ((double)red) / 255.0,
        ((double)green) / 255.0,
        ((double)blue) / 255.0,
        ((double)alpha) / 255.0);
    cr->rectangle(0, 0, width, height);
    cr->fill();
    tex::Graphics2D_cairo g2(cr);
    render->draw(g2, padding, padding);
    delete render;
    size_t buffer_size = width * height * 4 * sizeof(uint8_t);
    std::memcpy(image_ptr, surface->get_data(), buffer_size);
}