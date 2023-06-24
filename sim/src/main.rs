use clap::{Args, Parser};
use std::fs;

#[derive(Default)]
struct Cycles {
    count: usize,
}

impl Cycles {
    fn add(&mut self, num: usize) {
        self.count += num;
    }

    fn get(&self) -> usize {
        self.count
    }
}

struct LRU {
    list: Vec<usize>,
}

impl LRU {
    fn new(size: usize) -> Self {
        Self {
            list: (0..size).collect(),
        }
    }

    fn access(&mut self, idx: usize) {
        let pos = self.list.iter().position(|n| *n == idx).unwrap();
        self.list.remove(pos);
        self.list.push(idx);
    }

    fn lru(&self) -> usize {
        self.list[0]
    }
}

enum CacheResult {
    Hit,
    Miss,
}

#[derive(Default, Clone, Copy)]
struct Way {
    tag: Option<u32>,
}

struct Set {
    lru: LRU,
    ways: Vec<Way>,
}

impl Set {
    fn new(config: &MemConfig) -> Self {
        Self {
            lru: LRU::new(config.associativity),
            ways: vec![Default::default(); config.associativity],
        }
    }

    fn access(&mut self, tag: u32) -> CacheResult {
        let way_idx = self.ways.iter().position(|way| way.tag == Some(tag));
        if let Some(way) = way_idx {
            // Cache hit
            self.lru.access(way);
            CacheResult::Hit
        } else {
            // Cache miss
            let lru_idx = self.lru.lru();
            self.ways[lru_idx].tag = Some(tag);
            CacheResult::Miss
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    fw_file: String,
    #[command(flatten)]
    mem_config: MemConfig,
}

struct Cache {
    sets: Vec<Set>,
    line_bits: u32,
    set_bits: u32,
}

impl Cache {
    fn new(config: &MemConfig) -> Self {
        let set_size = config.associativity * config.line_size;
        assert_eq!(
            config.cache_size % set_size,
            0,
            "Cache size not divisible by set size"
        );
        let num_sets = config.cache_size / set_size;
        Self {
            sets: (0..num_sets).map(|_| Set::new(config)).collect(),
            line_bits: config.line_size.ilog2(),
            set_bits: num_sets.ilog2(),
        }
    }

    fn access(&mut self, addr: u32) -> CacheResult {
        let set_idx = (addr >> self.line_bits) & ((!0) >> (32 - self.set_bits));
        let tag = addr >> (self.line_bits + self.set_bits);
        self.sets[set_idx as usize].access(tag)
    }
}

struct Mem {
    data: Vec<u8>,
    icache: Cache,
    dcache: Cache,

    l1d_hit_cycles: usize,
    l1d_miss_cycles: usize,
    l1i_miss_cycles: usize,
}

impl Mem {
    fn new(init_data: &[u8], config: MemConfig) -> Self {
        let mut data = vec![0; config.mem_size];
        data[0..init_data.len()].copy_from_slice(init_data);
        Self {
            data,
            icache: Cache::new(&config),
            dcache: Cache::new(&config),

            l1d_hit_cycles: config.l1d_hit_cycles,
            l1d_miss_cycles: config.l1d_miss_cycles,
            l1i_miss_cycles: config.l1i_miss_cycles,
        }
    }

    fn dcache_access(&mut self, cycles: &mut Cycles, addr: u32) {
        assert!(
            addr < self.data.len() as u32,
            "out of bounds memory access at address 0x{addr:08x}"
        );
        match self.dcache.access(addr) {
            CacheResult::Hit => cycles.add(self.l1d_hit_cycles),
            CacheResult::Miss => cycles.add(self.l1d_miss_cycles),
        }
    }

    fn lw(&mut self, cycles: &mut Cycles, addr: u32) -> u32 {
        self.dcache_access(cycles, addr);
        let addr = addr as usize;
        self.data[addr] as u32
            | ((self.data[addr + 1] as u32) << 8)
            | ((self.data[addr + 2] as u32) << 16)
            | ((self.data[addr + 3] as u32) << 24)
    }

    fn lh(&mut self, cycles: &mut Cycles, addr: u32) -> u16 {
        self.dcache_access(cycles, addr);
        let addr = addr as usize;
        self.data[addr] as u16 | ((self.data[addr + 1] as u16) << 8)
    }

    fn lb(&mut self, cycles: &mut Cycles, addr: u32) -> u8 {
        self.dcache_access(cycles, addr);
        self.data[addr as usize]
    }

    fn sw(&mut self, cycles: &mut Cycles, addr: u32, val: u32) {
        self.dcache_access(cycles, addr);
        let addr = addr as usize;
        self.data[addr] = (val & 0x000000FF) as u8;
        self.data[addr + 1] = ((val & 0x0000FF00) >> 8) as u8;
        self.data[addr + 2] = ((val & 0x00FF0000) >> 16) as u8;
        self.data[addr + 3] = ((val & 0xFF000000) >> 24) as u8;
    }

    fn sh(&mut self, cycles: &mut Cycles, addr: u32, val: u16) {
        self.dcache_access(cycles, addr);
        let addr = addr as usize;
        self.data[addr] = (val & 0x00FF) as u8;
        self.data[addr + 1] = ((val & 0xFF00) >> 8) as u8;
    }

    fn sb(&mut self, cycles: &mut Cycles, addr: u32, val: u8) {
        self.dcache_access(cycles, addr);
        self.data[addr as usize] = val;
    }

    fn load_ins(&mut self, cycles: &mut Cycles, pc: u32) -> u32 {
        match self.icache.access(pc) {
            CacheResult::Hit => {}
            CacheResult::Miss => cycles.add(self.l1i_miss_cycles),
        }
        let addr = pc as usize;
        self.data[addr] as u32
            | ((self.data[addr + 1] as u32) << 8)
            | ((self.data[addr + 2] as u32) << 16)
            | ((self.data[addr + 3] as u32) << 24)
    }
}

#[derive(Args, Debug)]
struct MemConfig {
    #[arg(long)]
    mem_size: usize,
    #[arg(long)]
    cache_size: usize,
    #[arg(long)]
    line_size: usize,
    #[arg(long)]
    associativity: usize,
    #[arg(long)]
    l1d_hit_cycles: usize,
    #[arg(long)]
    l1d_miss_cycles: usize,
    #[arg(long)]
    l1i_miss_cycles: usize,
}

#[derive(Debug, Clone, Copy)]
enum Op {
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
    fn from_opcode(opcode: u32) -> Self {
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

fn read_imm_i(inst: u32) -> u32 {
    (inst as i32 >> 20) as u32
}

fn read_imm_s(inst: u32) -> u32 {
    (read_imm_i(inst) & !0b11111) | ((inst >> 7) & 0b11111)
}

fn read_imm_b(inst: u32) -> u32 {
    let low = (inst >> 7) & 0b11111 & !1;
    let mid = (inst << 4) & (1 << 11);
    let high = read_imm_i(inst) & !0b11111 & !(1 << 11);
    low | mid | high
}

fn read_imm_u(inst: u32) -> u32 {
    inst & !(0xFFF)
}

fn read_imm_j(inst: u32) -> u32 {
    let a = read_imm_i(inst) & 0xFFF007FE;
    let b = inst & 0x000FF000;
    let c = (inst & (1 << 20)) >> 9;
    a | b | c
}

fn funct3(inst: u32) -> u32 {
    (inst >> 12) & 0b111
}

fn rs1(inst: u32) -> u32 {
    (inst >> 15) & 0b11111
}

fn rs2(inst: u32) -> u32 {
    (inst >> 20) & 0b11111
}

fn rd(inst: u32) -> u32 {
    (inst >> 7) & 0b11111
}

fn funct7(inst: u32) -> u32 {
    inst >> 25
}

struct Core {
    mem: Mem,
    cycles: Cycles,
    pc: u32,
    regs: [u32; 32],
}

impl Core {
    fn new(fw_data: &[u8], mem_config: MemConfig) -> Self {
        Self {
            mem: Mem::new(fw_data, mem_config),
            cycles: Default::default(),
            pc: 0,
            regs: [0; 32],
        }
    }

    fn reg(&self, id: u32) -> u32 {
        match id {
            0 => 0,
            _ => self.regs[id as usize],
        }
    }

    fn execute_one(&mut self) -> bool {
        let ins = self.mem.load_ins(&mut self.cycles, self.pc);
        let op = Op::from_opcode(ins & 0b1111111);
        println!(
            "Cycle:{:04} PC:{:08x} {:032b} {:?}",
            self.cycles.get(),
            self.pc,
            ins,
            op
        );
        match op {
            Op::Op => {
                self.cycles.add(1);
                let rs1_val = self.reg(rs1(ins));
                let rs2_val = self.reg(rs2(ins));
                self.regs[rd(ins) as usize] = match funct3(ins) {
                    0b000 => {
                        if funct7(ins) == 0 {
                            rs1_val.overflowing_add(rs2_val).0
                        } else {
                            rs1_val.overflowing_sub(rs2_val).0
                        }
                    }
                    0b001 => rs1_val << rs2_val,
                    0b010 => {
                        if (rs1_val as i32) < (rs2_val as i32) {
                            1
                        } else {
                            0
                        }
                    }
                    0b011 => {
                        if rs1_val < rs2_val {
                            1
                        } else {
                            0
                        }
                    }
                    0b100 => rs1_val ^ rs2_val,
                    0b101 => {
                        if funct7(ins) == 0 {
                            rs1_val >> rs2_val
                        } else {
                            ((rs1_val as i32) >> rs2_val) as u32
                        }
                    }
                    0b110 => rs1_val | rs2_val,
                    0b111 => rs1_val & rs2_val,
                    _ => unreachable!(),
                };
                self.pc += 4;
            }
            Op::OpImm => {
                self.cycles.add(1);
                let rs1_val = self.reg(rs1(ins));
                let imm = read_imm_i(ins);
                self.regs[rd(ins) as usize] = match funct3(ins) {
                    0b000 => rs1_val.overflowing_add(imm).0,
                    0b001 => rs1_val << rs2(ins),
                    0b010 => {
                        if (rs1_val as i32) < (imm as i32) {
                            1
                        } else {
                            0
                        }
                    }
                    0b011 => {
                        if rs1_val < imm {
                            1
                        } else {
                            0
                        }
                    }
                    0b100 => rs1_val ^ imm,
                    0b101 => {
                        if funct7(ins) == 0 {
                            rs1_val >> rs2(ins)
                        } else {
                            ((rs1_val as i32) >> rs2(ins)) as u32
                        }
                    }
                    0b110 => rs1_val | imm,
                    0b111 => rs1_val & imm,
                    _ => unreachable!(),
                };
                self.pc += 4;
            }
            Op::Lui => {
                self.cycles.add(1);
                self.regs[rd(ins) as usize] = read_imm_u(ins);
                self.pc += 4;
            }
            Op::Auipc => {
                self.cycles.add(1);
                self.regs[rd(ins) as usize] = read_imm_u(ins) + self.pc;
                self.pc += 4;
            }
            Op::Branch => {
                let src1 = self.reg(rs1(ins));
                let src2 = self.reg(rs2(ins));
                let should_branch = match funct3(ins) {
                    0b000 => src1 == src2,
                    0b001 => src1 != src2,
                    0b100 => (src1 as i32) < (src2 as i32),
                    0b101 => (src1 as i32) > (src2 as i32),
                    0b110 => src1 < src2,
                    0b111 => src1 > src2,
                    cond => panic!("Invalid branch condition: {cond:03b}"),
                };
                if should_branch {
                    self.pc = (self.pc as i32 + read_imm_b(ins) as i32) as u32;
                }
            }
            Op::Jal => {
                self.cycles.add(1);
                self.regs[rd(ins) as usize] = self.pc + 4;
                self.pc = (self.pc as i32 + read_imm_j(ins) as i32) as u32;
            }
            Op::Jalr => {
                self.cycles.add(1);
                self.regs[rd(ins) as usize] = self.pc + 4;
                self.pc = (self.reg(rs1(ins)) as i32 + read_imm_j(ins) as i32) as u32;
            }
            Op::Load => {
                let addr = (self.reg(rs1(ins)) as i32 + read_imm_i(ins) as i32) as u32;
                self.regs[rd(ins) as usize] = match funct3(ins) {
                    0b000 => self.mem.lb(&mut self.cycles, addr) as u32,
                    0b001 => self.mem.lh(&mut self.cycles, addr) as u32,
                    0b010 => self.mem.lw(&mut self.cycles, addr),
                    0b100 => self.mem.lb(&mut self.cycles, addr) as i8 as i32 as u32,
                    0b101 => self.mem.lh(&mut self.cycles, addr) as i16 as i32 as u32,
                    width => panic!("Invalid width for load: {width:03b}"),
                };
                self.pc += 4;
            }
            Op::Store => {
                let addr = (self.reg(rs1(ins)) as i32 + read_imm_s(ins) as i32) as u32;
                let src = self.reg(rs2(ins));
                match funct3(ins) {
                    0b000 => self.mem.sb(&mut self.cycles, addr, src as u8),
                    0b001 => self.mem.sh(&mut self.cycles, addr, src as u16),
                    0b010 => self.mem.sw(&mut self.cycles, addr, src),
                    width => panic!("Invalid width for load: {width:03b}"),
                };
                self.pc += 4;
            }
            Op::MiscMem => {
                self.cycles.add(1);
                self.pc += 4;
            }
        }
        true
    }
}

fn main() {
    let args = Cli::parse();
    let fw_data = fs::read(args.fw_file).unwrap();
    println!("Firmware size: {} bytes", fw_data.len());
    let mut core = Core::new(&fw_data, args.mem_config);

    while core.execute_one() {}
}
