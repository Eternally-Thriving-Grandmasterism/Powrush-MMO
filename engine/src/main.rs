// Powrush-MMO — MercyEngine entry
// Runs on desktop, web (WASM), mobile

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // Config + render loop (procedural lattice UI)
    let mut renderer = MercyRenderer::new(device, queue, &surface);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => renderer.render(),
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}
