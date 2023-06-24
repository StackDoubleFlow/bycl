mod exec;
mod mem;
mod op;

use clap::Parser;
use exec::Core;
use mem::MemConfig;
use std::fs;

#[derive(Parser, Debug)]
struct Cli {
    fw_file: String,
    #[command(flatten)]
    mem_config: MemConfig,
}

fn main() {
    let args = Cli::parse();
    let fw_data = fs::read(args.fw_file).unwrap();
    println!("Firmware size: {} bytes", fw_data.len());
    let mut core = Core::new(&fw_data, args.mem_config);

    while core.execute_one() {}
}
