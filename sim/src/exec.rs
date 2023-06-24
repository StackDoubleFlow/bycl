use crate::mem::{Mem, MemConfig};
use crate::op::Op;

#[derive(Default)]
pub struct Cycles {
    count: usize,
}

impl Cycles {
    pub fn add(&mut self, num: usize) {
        self.count += num;
    }

    fn get(&self) -> usize {
        self.count
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

pub struct Core {
    mem: Mem,
    cycles: Cycles,
    pc: u32,
    regs: [u32; 32],
}

impl Core {
    pub fn new(fw_data: &[u8], mem_config: MemConfig) -> Self {
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

    pub fn execute_one(&mut self) -> bool {
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
                } else {
                    self.pc += 4;
                }
            }
            Op::Jal => {
                self.cycles.add(1);
                self.regs[rd(ins) as usize] = self.pc + 4;
                let dest = (self.pc as i32 + read_imm_j(ins) as i32) as u32;
                if self.pc == dest {
                    println!("Infinite loop.");
                    return false;
                }
                self.pc = dest;
            }
            Op::Jalr => {
                self.cycles.add(1);
                self.regs[rd(ins) as usize] = self.pc + 4;
                self.pc = (self.reg(rs1(ins)) as i32 + read_imm_i(ins) as i32) as u32;
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
