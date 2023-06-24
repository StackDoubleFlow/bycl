use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const PIXEL_WIDTH: u32 = 256;
const PIXEL_HEIGHT: u32 = 256;
const SCALE: u32 = 2;

const SCALED_WIDTH: u32 = PIXEL_WIDTH * SCALE;
const SCALED_HEIGHT: u32 = PIXEL_HEIGHT * SCALE;

/// Needs to be called on main thread
pub fn init() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(PhysicalSize::new(SCALED_WIDTH, SCALED_HEIGHT))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            _ => (),
        }
    });
}
