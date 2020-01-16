
use winit::{
    event::{
        Event,
        WindowEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState,
    },
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    window::{WindowBuilder},
};
use imgui::*;
use imgui_wgpu::Renderer;
// use imgui_winit_support;
use std::time::Instant;

mod wrapper;

use wrapper::*;

extern crate pretty_env_logger;
use log::{
    LevelFilter::{
        Trace,
        Warn,
    },
    trace, 
    debug,
    info,
    warn,
    error, 
};

fn main() {

// Initialize the logging system, for now we set the log-level to 
// Trace, instead of tying it to the environment. 
pretty_env_logger::formatted_timed_builder()
  .filter_level(Trace)
  .filter_module("wgpu_native", Warn)
  .filter_module("gfx_backend_vulkan", Warn)
  .init();

trace!("Creating winit event loop.");
let event_loop = EventLoop::new();

trace!("Creating initial window.");

let window = WindowBuilder::new()
.build(&event_loop)
.unwrap();

trace!("constructing GPU state"); 
let mut gpu_state = gpu::State::new(&window);

// let (window, mut size, surface, hidpi_factor) 
 //   = wrapper::window::init_window(&event_loop); 

//trace!("Creating gpu adapter."); 
// An adapter is basically a reference to the GPU we intend to use
//let adapter = wrapper::gpu::get_adapter();



trace!("Entering Event Loop");
event_loop.run(move |event, _, control_flow| {
match event {
    Event::WindowEvent {
        ref event,
        window_id,
    } if window_id == window.id() => match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(physical_size) => {
                gpu_state.resize(physical_size);
            },
        WindowEvent::KeyboardInput {
            input,
            ..
        } => {
            match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => { 
                    trace!("Quitting"); 
                    *control_flow = ControlFlow::Exit
                },
                _ => *control_flow = ControlFlow::Wait,
            }
        }
        _ => *control_flow = ControlFlow::Wait,
    }
    Event::MainEventsCleared => {
        gpu_state.update();
        gpu_state.render(); 
        *control_flow = ControlFlow::Wait
    }
    _ => *control_flow = ControlFlow::Wait,
}
});
}

/*
fn main() {
    pretty_env_logger::init();

    // Set up window and GPU
   

    let (mut device, mut queue) 
      = wrapper::gpu::get_device_queue(adapter);
    // Set up swap chain
    let mut sc_desc = wrapper::gpu::setup_swap_chain(size);
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = wrapper::gui::create_context();
    let (mut platform, font_size) = 
        wrapper::gui::init_platform(&mut imgui, &window, hidpi_factor);
    // Set up dear imgui wgpu renderer
    //
    let clear_color = wgpu::Color { r: 0.2, g: 0.2, b: 0.3, a: 1.0 };
    let mut renderer = 
        Renderer::new(
            &mut imgui,
            &device, 
            &mut queue, 
            sc_desc.format, 
            Some(clear_color),
        );

    let mut last_frame = Instant::now();
    let mut demo_open = true;

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                size = window
                    .inner_size();
                    //.to_physical(hidpi_factor);

                sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::NoVsync
                };

                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::WindowEvent {
               event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                },
                ..
            } |
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta = now - last_frame;
                let delta_s = delta.as_micros();
                last_frame = now;

                let frame = swap_chain.get_next_texture();
                platform.prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                {
                    let window = imgui::Window::new(im_str!("Hello world"));
                    window
                        .size([300.0, 100.0], Condition::FirstUseEver)
                        .build(&ui, || {
                            ui.text(im_str!("Hello world!"));
                            ui.text(im_str!("This...is...imgui-rs on WGPU!"));
                            ui.separator();
                            let mouse_pos = ui.io().mouse_pos;
                            ui.text(im_str!(
                                "Mouse Position: ({:.1},{:.1})",
                                mouse_pos[0],
                                mouse_pos[1]
                            ));
                        });

                    let window = imgui::Window::new(im_str!("Hello too"));
                    window
                        .size([400.0, 200.0], Condition::FirstUseEver)
                        .position([400.0, 200.0], Condition::FirstUseEver)
                        .build(&ui, || {
                            ui.text(im_str!("Frametime: {}us", delta_s));
                        });

                    ui.show_demo_window(&mut demo_open);
                }

                let mut encoder: wgpu::CommandEncoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                platform.prepare_render(&ui, &window);
                renderer
                    .render(ui, &mut device, &mut encoder, &frame.view)
                    .expect("Rendering failed");

                queue.submit(&[encoder.finish()]);
            },
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}
*/