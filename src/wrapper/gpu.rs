
use winit::{
    event::{
        WindowEvent
    },
    window::{
        Window,
    },
    dpi::{
        PhysicalSize,
    },
};
use wgpu;

/// Stores the current state of the gpu
pub struct State {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    physical_size: winit::dpi::PhysicalSize<u32>,
    logical_size: winit::dpi::LogicalSize<f32>,
    scale_factor: f64, 
}

impl State {
    pub fn new(window: &Window) -> Self {
     
        log::trace!("creating gpu adapter with default options"); 
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                ..Default::default()
        }).unwrap(); 

        State::new_with_adapter(window, adapter)
    }

    pub fn new_with_adapter(window: &Window, adapter : wgpu::Adapter)  -> Self {

        let physical_size = window.inner_size();

        log::trace!("Creating window surface"); 
        let surface = wgpu::Surface::create(window);

        log::trace!("Requesting device and queue"); 
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        });

        log::trace!("Contructing swap chain");
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: physical_size.width as u32,
            height: physical_size.height as u32,
            present_mode: wgpu::PresentMode::NoVsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let scale_factor = window.scale_factor();
        let logical_size = physical_size.to_logical(scale_factor);
        Self{
            surface, 
            adapter, 
            device, 
            queue, 
            sc_desc, 
            swap_chain, 
            physical_size, 
            logical_size, 
            scale_factor
        }
    }

    pub fn resize(&mut self, physical_size: &winit::dpi::PhysicalSize<u32>) {
        //let physical_size = new_size.to_physical(self.hidpi_factor);
        self.physical_size = *physical_size;
        self.sc_desc.width = physical_size.width;
        self.sc_desc.height = physical_size.height;
        self.logical_size = physical_size.to_logical(self.scale_factor);
        self.swap_chain = self.device.create_swap_chain(
            &self.surface, 
            &self.sc_desc);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) {
        let frame = self.swap_chain.get_next_texture();
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor{
                todo: 0,
            });
        {
            let _render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Clear,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 0.5,
                            },
                        }
                    ],
                    depth_stencil_attachment: None,
            });
        }
         self.queue.submit(&[
            encoder.finish()
        ]);
    }
}
 
/*
/// Requests a low power GPU adapter from wgpu
pub fn get_adapter() -> wgpu::Adapter {
    wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        backends: wgpu::BackendBit::PRIMARY,
    }).unwrap()
}

pub fn get_device_queue(adapter : wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    })
}

pub fn setup_swap_chain(size : winit::dpi::PhysicalSize<u32>) -> wgpu::SwapChainDescriptor {
    wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::NoVsync,
    }
}
*/