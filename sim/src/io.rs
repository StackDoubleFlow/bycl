use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionId(usize);

pub struct MmioPort {
    pub tx: Sender<(u32, TransactionId)>,
    pub rx: Receiver<(u32, TransactionId)>,
}

pub struct Mmio {
    ports: Vec<Option<MmioPort>>,
    port_data: Vec<u32>,
    next_tid: TransactionId,
}

impl Mmio {
    pub fn new(num_ports: usize) -> Self {
        let mut ports = Vec::with_capacity(num_ports);
        for _ in 0..num_ports {
            ports.push(None);
        }
        Self {
            ports,
            port_data: vec![0; num_ports],
            next_tid: TransactionId(0),
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

    pub fn update_ports(&mut self) {
        for (idx, port) in self.ports.iter().enumerate() {
            if let Some(port) = port {
                while let Ok((new_data, _)) = port.rx.try_recv() {
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
            port.tx.send((val, self.next_tid)).unwrap();
            self.next_tid.0 += 1;
        }
    }
}

pub struct ColumnDisplay {
    cur_data: u32,
    queue: VecDeque<(u32, TransactionId)>,
    data_rx: Receiver<(u32, TransactionId)>,
    submit_rx: Receiver<(u32, TransactionId)>,
    columns: Vec<u32>,
}

impl ColumnDisplay {
    pub fn new(data_port: MmioPort, col_port: MmioPort, num_columns: u32) -> Self {
        Self {
            cur_data: 0,
            queue: VecDeque::new(),
            data_rx: data_port.rx,
            submit_rx: col_port.rx,
            columns: vec![0; num_columns as usize],
        }
    }

    fn print_display(&self) {
        let mut str = String::with_capacity(self.columns.len());
        for row in (0..32).rev() {
            for &col in self.columns.iter() {
                let pixel = (col >> row) & 1 != 0;
                if pixel {
                    str.push_str("██");
                } else {
                    str.push_str("  ");
                }
            }
            println!("{str}");
            str.clear();
        }
    }

    pub fn update(&mut self) {
        while let Ok(packet) = self.data_rx.try_recv() {
            self.queue.push_back(packet);
        }

        let mut changed = false;
        while let Ok((col, submit_id)) = self.submit_rx.try_recv() {
            while let Some((_, data_id)) = self.queue.front() {
                if *data_id < submit_id {
                    self.cur_data = self.queue.pop_front().unwrap().0;
                }
            }
            let col = col as usize;
            if col >= self.columns.len() {
                panic!(
                    "Tried to write to column {col} when there are only {} columns",
                    self.columns.len()
                );
            }
            if self.columns[col] != self.cur_data {
                self.columns[col] = self.cur_data;
                changed = true;
            }
        }

        if changed {
            self.print_display();
        }
    }
}
