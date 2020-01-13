
use imgui::*;
use imgui_winit_support;
use winit; 

pub fn create_context() -> imgui::Context {
   imgui::Context::create()
}

pub fn init_platform(
    imgui : &mut imgui::Context, 
    window : &winit::window::Window, 
    hidpi_factor : f64,
) -> (imgui_winit_support::WinitPlatform, f32) {
    let mut platform = imgui_winit_support::WinitPlatform::init(imgui);
    platform.attach_window(imgui.io_mut(), &window);
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            })
        }
    ]);
    (platform, font_size)
}

