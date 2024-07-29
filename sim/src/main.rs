mod exec;
mod mem;
mod op;
mod io;

use clap::Parser;
use exec::{Core, CoreConfig};
use mem::MemConfig;
use std::{fs, thread};

#[derive(Parser, Debug)]
struct Cli {
    fw_file: String,
    #[command(flatten)]
    mem_config: MemConfig,
    #[command(flatten)]
    core_config: CoreConfig,
}

fn main() {
    let args = Cli::parse();

    thread::spawn(|| {
        let fw_data = fs::read(args.fw_file).unwrap();
        println!("Firmware size: {} bytes", fw_data.len());

        let mmio = io::Mmio::new(16);

        let mut core = Core::new(&fw_data, args.mem_config, args.core_config, mmio);
        while core.execute_one() {}
    });

    io::init();
}
