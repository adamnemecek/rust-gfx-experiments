use winit::{
    event_loop::{
        EventLoop,
    },
    dpi::LogicalSize,
    dpi::PhysicalSize,
    window::Window,
};
use wgpu;

pub fn init_window(event_loop : &EventLoop<()>) 
       -> (Window, PhysicalSize<u32> , wgpu::Surface, f64) {

        let version = env!("CARGO_PKG_VERSION");

        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize { width: 1280.0, height: 720.0 });
        window.set_title(&format!("imgui-wgpu {}", version));
        let hidpi_factor = window.scale_factor(); 
        let size = window .inner_size();

        let surface = wgpu::Surface::create(&window);

        (window, size, surface, hidpi_factor)
}