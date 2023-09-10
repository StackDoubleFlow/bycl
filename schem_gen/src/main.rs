use clap::Parser;
use redstone_schem::world::{BlockPos, World};
use std::fs;

const L1I_SIZE: usize = 1024;
const INS_WORD: usize = 4;

/// Generate schematics for the bycl CPU
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the binary file to convert
    input: String,
    /// Path to the schematic file to output
    #[arg(short, default_value = "bycl_program.schem")]
    output: String,
}

fn generate_l1i(data: &[u8]) -> World {
    let mut world = World::new(63, 72, 64);
    let rep_on = world.add_block("repeater[powered=true,facing=west,locked=true]");
    let rep_off = world.add_block("repeater[powered=false,facing=west,locked=true]");
    for word_idx in 0..(data.len() / INS_WORD) {
        assert!(word_idx < (L1I_SIZE / INS_WORD), "exceeded L1I size");
        let line_idx = word_idx / 32;
        let z_pos = 56 - line_idx * 8;
        let idx_in_line = word_idx % 32;
        let x_pos = 62 - (idx_in_line * 2);
        for layer_idx in 0..4 {
            let byte = data[word_idx * 4 + layer_idx];
            for bit_idx in 0..8 {
                let y_pos = layer_idx * 19 + bit_idx * 2;
                let pos = BlockPos::new(x_pos, y_pos, z_pos);

                let bit = (byte & (1 << bit_idx)) != 0;
                let block = if bit { rep_on } else { rep_off };

                world.set_block(pos, block);
            }
        }
    }
    world
}

fn main() {
    let args = Args::parse();
    let data = fs::read(args.input).unwrap();
    let world = generate_l1i(&data);
    world.save_schematic(&args.output, -63, -83, 0);
}
