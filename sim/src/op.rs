#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub enum Op {
    OpImm,
    Lui,
    Auipc,
    Op,
    Jal,
    Jalr,
    Branch,
    Load,
    Store,
    MiscMem,
}

impl Op {
    pub fn from_opcode(opcode: u32) -> Self {
        match opcode {
            0b0110111 => Op::Lui,
            0b0010111 => Op::Auipc,
            0b1101111 => Op::Jal,
            0b1100111 => Op::Jalr,
            0b1100011 => Op::Branch,
            0b0000011 => Op::Load,
            0b0100011 => Op::Store,
            0b0010011 => Op::OpImm,
            0b0110011 => Op::Op,
            0b0001111 => Op::MiscMem,
            _ => panic!("invalid opcode: {:07b}", opcode),
        }
    }
}
