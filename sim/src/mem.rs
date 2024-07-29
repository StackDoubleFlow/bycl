use crate::exec::Cycles;
use clap::Args;

struct Lru {
    list: Vec<usize>,
}

impl Lru {
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
    lru: Lru,
    ways: Vec<Way>,
}

impl Set {
    fn new(config: &MemConfig) -> Self {
        Self {
            lru: Lru::new(config.l1_associativity),
            ways: vec![Default::default(); config.l1_associativity],
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

struct Cache {
    sets: Vec<Set>,
    line_bits: u32,
    set_bits: u32,
}

impl Cache {
    fn new(config: &MemConfig) -> Self {
        let set_size = config.l1_associativity * config.l1_line_size;
        assert_eq!(
            config.l1_size % set_size,
            0,
            "Cache size not divisible by set size"
        );
        let num_sets = config.l1_size / set_size;
        Self {
            sets: (0..num_sets).map(|_| Set::new(config)).collect(),
            line_bits: config.l1_line_size.ilog2(),
            set_bits: num_sets.ilog2(),
        }
    }

    fn access(&mut self, addr: u32) -> CacheResult {
        let set_idx = (addr >> self.line_bits) & ((!0) >> (32 - self.set_bits));
        let tag = addr >> (self.line_bits + self.set_bits);
        self.sets[set_idx as usize].access(tag)
    }
}

pub struct Mem {
    data: Vec<u8>,
    icache: Cache,
    dcache: Cache,

    l1d_hit_cycles: usize,
    l1d_miss_cycles: usize,
    l1i_miss_cycles: usize,
}

impl Mem {
    pub fn new(init_data: &[u8], config: MemConfig) -> Self {
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

    pub fn load(&mut self, cycles: &mut Cycles, addr: u32) -> u32 {
        self.dcache_access(cycles, addr);
        let addr = addr as usize;
        self.data[addr] as u32
            | ((self.data[addr + 1] as u32) << 8)
            | ((self.data[addr + 2] as u32) << 16)
            | ((self.data[addr + 3] as u32) << 24)
    }

    pub fn store(&mut self, cycles: &mut Cycles, addr: u32, val: u32) {
        self.dcache_access(cycles, addr);
        let addr = addr as usize;
        self.data[addr] = (val & 0x000000FF) as u8;
        self.data[addr + 1] = ((val & 0x0000FF00) >> 8) as u8;
        self.data[addr + 2] = ((val & 0x00FF0000) >> 16) as u8;
        self.data[addr + 3] = ((val & 0xFF000000) >> 24) as u8;
    }

    pub fn load_ins(&mut self, cycles: &mut Cycles, pc: u32) -> u32 {
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
pub struct MemConfig {
    #[arg(long)]
    mem_size: usize,
    #[arg(long)]
    l1_size: usize,
    #[arg(long)]
    l1_line_size: usize,
    #[arg(long)]
    l1_associativity: usize,
    #[arg(long)]
    l1d_hit_cycles: usize,
    #[arg(long)]
    l1d_miss_cycles: usize,
    #[arg(long)]
    l1i_miss_cycles: usize,
}
