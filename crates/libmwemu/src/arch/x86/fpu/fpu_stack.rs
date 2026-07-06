use crate::fpu::F80;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct FPUStack {
    regs: [F80; 8],
    top: u8,
    depth: u8,
    log: bool,
    invalid_flag: u8,
}

impl Default for FPUStack {
    fn default() -> Self {
        Self::new()
    }
}

impl FPUStack {
    pub fn new() -> Self {
        Self {
            regs: [F80::new(); 8],
            top: 0,
            depth: 0,
            log: false,
            invalid_flag: 0,
        }
    }

    pub fn len(&self) -> usize {
        8
    }

    pub fn print(&self) {
        info!("top: {}", self.top);
        info!("depth: {}", self.depth);
        for i in (0..8).rev() {
            info!(
                "ST({}) 0x{:x} {}",
                i,
                self.regs[i].get(),
                self.regs[i].get_f64()
            );
        }
        info!("invalid_flag: {:b}", self.invalid_flag);
    }

    pub fn get_top(&mut self) -> u8 {
        self.top
    }

    pub fn inc_top(&mut self) {
        self.top = (self.top + 1) & 7;
    }

    pub fn dec_top(&mut self) {
        self.top = (self.top.wrapping_sub(1)) & 7;
    }

    pub fn set_invalid(&mut self, idx: usize) {
        assert!(idx < 8);
        self.invalid_flag |= 1 << idx;
    }

    pub fn clear_invalid(&mut self, idx: usize) {
        assert!(idx < 8);
        self.invalid_flag &= !(1 << idx);
    }

    pub fn is_invalid(&self, idx: usize) -> bool {
        assert!(idx < 8);
        (self.invalid_flag & (1 << idx)) != 0
    }

    pub fn push_f64(&mut self, value: f64) {
        if self.depth >= 8 {
            panic!("FPU stack overflow");
            // in linux: terminated by signal SIGFPE (Floating point exception)
        }
        self.top = (self.top.wrapping_sub(1)) & 7;
        self.regs[self.top as usize].set_f64(value);
        self.depth += 1;
        self.clear_invalid(self.top.into());
    }

    pub fn push_f80(&mut self, value: F80) {
        if self.depth >= 8 {
            panic!("FPU stack overflow");
            // in linux: terminated by signal SIGFPE (Floating point exception)
        }
        self.top = (self.top.wrapping_sub(1)) & 7;
        self.regs[self.top as usize].set(value.get());
        self.depth += 1;
        self.clear_invalid(self.top.into());
    }

    pub fn pop(&mut self) -> Option<F80> {
        if self.depth == 0 {
            return None;
        }
        let val = self.regs[self.top as usize];
        self.top = (self.top.wrapping_add(1)) & 7;
        self.depth -= 1;
        //println!("depth decremented to {}", self.depth);
        Some(val)
    }

    // only for tests.rs, this acces directly to index, it's not using self.top
    pub fn peek(&self, n: usize) -> F80 {
        self.regs[n]
    }

    pub fn get(&mut self, n: usize) -> F80 {
        let idx = (self.top as usize + n) % 8;

        if self.depth == 0 || n >= self.depth as usize {
            self.set_invalid(idx);
        }
        self.regs[idx]
    }

    pub fn get_ref(&mut self, n: usize) -> &F80 {
        let idx = (self.top as usize + n) % 8;

        if self.depth == 0 || n >= self.depth as usize {
            self.set_invalid(idx);
        }
        &self.regs[idx]
    }

    pub fn get_mut(&mut self, n: usize) -> Option<&mut F80> {
        let idx = (self.top as usize + n) % 8;

        if self.depth == 0 || n >= self.depth as usize {
            self.set_invalid(idx);
        }

        if self.is_invalid(idx) {
            self.regs[idx].set_real_indefinite();
            None
        } else {
            Some(&mut self.regs[idx])
        }
    }

    /*
    pub fn st0(&self) -> &F80 {
        &self.regs[self.top as usize]
    }*/

    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    pub fn physical_index(&self, n: usize) -> usize {
        (self.top as usize + n) % 8
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        let a = self.top as usize + a;
        let b = (self.top as usize + b) % 8;
        self.regs.swap(a, b);
    }
}
