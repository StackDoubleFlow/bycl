use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use std::sync::mpsc::{channel, Sender, Receiver};

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

pub struct MmioPort {
    pub tx: Sender<u32>,
    pub rx: Receiver<u32>,
}

pub struct Mmio {
    ports: Vec<Option<MmioPort>>,
    port_data: Vec<u32>,
}

impl Mmio {
    pub fn new(num_ports: usize) -> Self {
        let mut ports = Vec::with_capacity(num_ports);
        for _ in 0..num_ports {
            ports.push(None);
        }
        Self {
            ports,
            port_data: vec![0; num_ports]
        }
    }

    pub fn attach_port(&mut self, idx: usize) -> MmioPort {
        let (core_tx, device_rx) = channel();
        let (device_tx, core_rx) = channel();
        self.ports[idx] = Some(MmioPort {
            tx: core_tx,
            rx: core_rx,
        });
        MmioPort {
            tx: device_tx,
            rx: device_rx,
        }
    }

    fn update_ports(&mut self) {
        for (idx, port) in self.ports.iter().enumerate() {
            if let Some(port) = port {
                while let Ok(new_data) = port.rx.try_recv() {
                    self.port_data[idx] = new_data;
                }

            }
        }
    }

    pub fn load(&mut self, idx: usize) -> u32 {
        self.port_data[idx]
    }

    pub fn store(&mut self, idx: usize, val: u32) {
        if let Some(port) = &self.ports[idx] {
            port.tx.send(val);
        }
    }
}
