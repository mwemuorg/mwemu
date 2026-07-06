pub mod f80;
pub mod fpu_stack;

use crate::emu;
use f80::F80;
use fpu_stack::FPUStack;
use iced_x86::Register;

pub struct FPUState {
    pub fpu_control_word: u16, // Control Word
    pub fpu_status_word: u16,  // Status Word
    pub fpu_tag_word: u16,     // Tag Word
    pub reserved1: u8,
    pub fpu_opcode: u16, // Opcode
    pub rip: u64,        // Instruction Pointer
    pub rdp: u64,        // Data Pointer
    pub mxcsr: u32,      // SSE Control and Status
    pub mxcsr_mask: u32,
    pub st: FPUStack,         // FPU registers
    pub xmm: [u128; 16],      // XMM registers
    pub reserved2: [u8; 224], // Reserved
}

impl Default for FPUState {
    fn default() -> Self {
        Self::new()
    }
}

impl FPUState {
    pub fn new() -> Self {
        Self {
            fpu_control_word: 0,
            fpu_status_word: 0,
            fpu_tag_word: 0,
            reserved1: 0,
            fpu_opcode: 0,
            rip: 0,
            rdp: 0,
            mxcsr: 0,
            mxcsr_mask: 0,
            st: FPUStack::new(),
            xmm: [0; 16],
            reserved2: [0; 224],
        }
    }

    pub fn load(addr: u64, emu: &mut emu::Emu) -> FPUState {
        let mut state = FPUState::new();
        state.fpu_control_word = emu.maps.read_word(addr).unwrap();
        state.fpu_status_word = emu.maps.read_word(addr + 2).unwrap();
        state.fpu_tag_word = emu.maps.read_word(addr + 4).unwrap();
        state.fpu_opcode = emu.maps.read_word(addr + 6).unwrap();
        state.rip = emu.maps.read_qword(addr + 8).unwrap();
        state.rdp = emu.maps.read_qword(addr + 16).unwrap();
        state.mxcsr = emu.maps.read_dword(addr + 24).unwrap();
        state.mxcsr_mask = emu.maps.read_dword(addr + 28).unwrap();
        state
    }

    pub fn save(&self, addr: u64, emu: &mut emu::Emu) {
        emu.maps.write_word(addr, self.fpu_control_word); // FCW (offset 0)
        emu.maps.write_word(addr + 2, self.fpu_status_word); // FSW (offset 2)
        emu.maps.write_word(addr + 4, self.fpu_tag_word); // FTW (offset 4)
        emu.maps.write_word(addr + 6, self.fpu_opcode); // FOP (offset 6)
        emu.maps.write_qword(addr + 8, self.rip); // RIP (offset 8)
        emu.maps.write_qword(addr + 16, self.rdp); // RDP (offset 16)
        emu.maps.write_dword(addr + 24, self.mxcsr); // MXCSR (offset 24)
        emu.maps.write_dword(addr + 28, self.mxcsr_mask); // MXCSR_MASK (offset 28)
    }
}

#[derive(Clone)]
pub struct FPU {
    pub st: FPUStack,
    pub st_depth: u8,
    pub status: u16,
    pub tag: u16,
    pub stat: u16,
    pub ctrl: u16,
    pub ip: u64,
    pub err_off: u32,
    pub err_sel: u32,
    pub code_segment: u16,
    pub data_segment: u16,
    pub operand_ptr: u64,
    pub reserved: [u8; 14],
    pub reserved2: [u8; 96],
    pub xmm: [u128; 16],
    pub mxcsr: u32,
    pub fpu_control_word: u16,
    pub opcode: u16,
    pub trace: bool,
}

impl Default for FPU {
    fn default() -> Self {
        Self::new()
    }
}

impl FPU {
    pub fn new() -> FPU {
        FPU {
            st: FPUStack::new(),
            status: 0,
            st_depth: 0,
            tag: 0xffff,
            stat: 0,
            ctrl: 0x027f,
            ip: 0,
            err_off: 0,
            err_sel: 0,
            code_segment: 0,
            data_segment: 0,
            operand_ptr: 0,
            reserved: [0; 14],
            reserved2: [0; 96],
            xmm: [0; 16],
            mxcsr: 0,
            fpu_control_word: 0,
            opcode: 0,
            trace: false,
        }
    }

    pub fn clear(&mut self) {
        self.st_depth = 0;
        self.st = FPUStack::new();
        self.tag = 0xffff;
        self.stat = 0;
        self.ctrl = 0x037f;
        self.ip = 0;
        self.err_off = 0;
        self.err_sel = 0;
        self.code_segment = 0;
        self.data_segment = 0;
        self.operand_ptr = 0;
        self.reserved = [0; 14];
        self.reserved2 = [0; 96];
        self.xmm = [0; 16];
        self.fpu_control_word = 0;
    }

    pub fn get_status_c0(&self) -> bool {
        (self.status & (1 << 8)) != 0
    }

    pub fn get_status_c1(&self) -> bool {
        (self.status & (1 << 9)) != 0
    }

    pub fn get_status_c2(&self) -> bool {
        (self.status & (1 << 10)) != 0
    }

    pub fn get_status_c3(&self) -> bool {
        (self.status & (1 << 14)) != 0
    }

    pub fn get_status_busy(&self) -> bool {
        (self.status & (1 << 15)) != 0
    }

    pub fn set_status_c0(&mut self, val: bool) {
        if val {
            self.status |= 1 << 8;
        } else {
            self.status &= !(1 << 8);
        }
    }

    pub fn set_status_c1(&mut self, val: bool) {
        if val {
            self.status |= 1 << 9;
        } else {
            self.status &= !(1 << 9);
        }
    }

    pub fn set_status_c2(&mut self, val: bool) {
        if val {
            self.status |= 1 << 10;
        } else {
            self.status &= !(1 << 10);
        }
    }

    pub fn set_status_c3(&mut self, val: bool) {
        if val {
            self.status |= 1 << 14;
        } else {
            self.status &= !(1 << 14);
        }
    }

    pub fn set_status_busy(&mut self, val: bool) {
        if val {
            self.status |= 1 << 15;
        } else {
            self.status &= !(1 << 15);
        }
    }

    pub fn get_top(&mut self) -> u8 {
        self.st.get_top()
    }

    pub fn get_depth(&mut self) -> u8 {
        self.st.get_depth()
    }

    pub fn set_ctrl(&mut self, ctrl: u16) {
        self.ctrl = ctrl;
    }

    pub fn get_ctrl(&self) -> u16 {
        self.ctrl
    }

    pub fn do_trace(&mut self) {
        self.trace = true;
    }

    pub fn set_ip(&mut self, ip: u64) {
        self.ip = ip;
        if self.trace {
            self.st.print();
        }
    }

    pub fn get_env32(&self) -> Vec<u32> {
        let mut r: Vec<u32> = Vec::new();
        let mut r1: u32 = self.tag as u32;
        r1 <<= 16;
        r1 += self.ctrl as u32;
        r.push(r1);
        r.push(0xffff0000);
        r.push(0xffffffff);
        r.push(self.ip as u32);
        r
    }

    pub fn get_env64(&self) -> Vec<u64> {
        let mut r: Vec<u64> = Vec::new();
        let mut r1: u64 = self.tag as u64;
        r1 <<= 16;
        r1 += self.ctrl as u64;
        r.push(r1);
        r.push(0xffff0000);
        r.push(0xffffffff);
        r.push(self.ip);
        r
    }

    pub fn print(&mut self) {
        log::trace!("---- fpu ----");
        for i in 0..self.st.len() {
            log::trace!("st({}): {}", i, self.st.get(i).get());
        }

        log::trace!("stat: 0x{:x}", self.stat);
        log::trace!("ctrl: 0x{:x}", self.ctrl);
        log::trace!("eip:  0x{:x}", self.ip);

        log::trace!("--------");
    }

    pub fn set_st(&mut self, i: usize, value: f64) {
        if let Some(st) = self.st.get_mut(i) { st.set_f64(value) }
    }

    pub fn set_st_u80(&mut self, i: usize, value: u128) {
        if let Some(st) = self.st.get_mut(i) { st.set(value) }
    }

    // only use from test.rs
    pub fn peek_st_f64(&mut self, i: usize) -> f64 {
        self.st.peek(i).get_f64()
    }

    // only use from test.rs
    pub fn peek_st_u80(&mut self, i: usize) -> u128 {
        self.st.peek(i).get()
    }

    pub fn peek_st_logical_f64(&mut self, n: usize) -> f64 {
        let idx = (self.st.get_top() as usize + n) % 8;
        self.peek_st_f64(idx)
    }

    pub fn peek_st_logical_u80(&mut self, n: usize) -> u128 {
        let idx = (self.st.get_top() as usize + n) % 8;
        self.peek_st_u80(idx)
    }

    pub fn get_st_u80(&mut self, i: usize) -> u128 {
        self.st.get(i).get()
    }

    pub fn get_st(&mut self, i: usize) -> f64 {
        self.st.get(i).get_f64()
    }

    pub fn xchg_st(&mut self, i: usize) {
        let i = i;
        self.st.swap(0, i);
    }

    pub fn clear_st(&mut self, i: usize) {
        if let Some(st) = self.st.get_mut(i) { st.set(0) }
    }

    pub fn neg_st(&mut self, i: usize) {
        if let Some(st) = self.st.get_mut(i) { st.neg() }
    }

    pub fn move_to_st0(&mut self, i: usize) {
        let v = self.st.get(i).get();
        if let Some(st) = self.st.get_mut(0) { st.set(v) }
    }

    pub fn add_to_st0(&mut self, i: usize) {
        let v = self.st.get(i);
        if let Some(st) = self.st.get_mut(0) { st.add(v) }
    }

    pub fn add(&mut self, i: usize, j: usize) {
        let v = self.st.get(j);
        if let Some(st) = self.st.get_mut(i) { st.add(v) }
    }

    pub fn sub(&mut self, i: usize, j: usize) {
        let v = self.st.get(j);
        if let Some(st) = self.st.get_mut(i) { st.sub(v) }
    }

    pub fn subr(&mut self, i: usize, j: usize) {
        let a = self.st.get(i);
        let mut b = self.st.get(j);
        b.sub(a);
        if let Some(st) = self.st.get_mut(i) { st.set(b.get()) }
    }

    pub fn push_f64(&mut self, value: f64) {
        self.st.push_f64(value);
    }

    pub fn push_f80(&mut self, value: F80) {
        self.st.push_f80(value);
    }

    pub fn get_st_f80_ref(&mut self, n: usize) -> &F80 {
        self.st.get_ref(n)
    }

    pub fn get_st_f80_copy(&mut self, n: usize) -> F80 {
        self.st.get(n)
    }

    pub fn is_empty(&mut self, a: usize) -> bool {
        self.st.get(a).is_empty()
    }

    pub fn pop2(&mut self) -> u128 {
        
        match self.st.pop() {
            Some(f80val) => f80val.get(),
            None => 0,
        }
    }

    pub fn pop_f64(&mut self) -> f64 {
        
        match self.st.pop() {
            Some(f80val) => f80val.get_f64(),
            None => 0.0,
        }
    }

    pub fn fyl2x(&mut self) {
        let v = self.st.get(1).get_f64() * self.st.get(0).get_f64().log2();
        if let Some(st) = self.st.get_mut(1) { st.set_f64(v) }
        self.st.pop();
    }

    pub fn fyl2xp1(&mut self) {
        let v = self.st.get(1).get_f64() * (self.st.get(0).get_f64().log2() + 1.0);
        if let Some(st) = self.st.get_mut(1) { st.set_f64(v) }
        self.st.pop();
    }

    pub fn check_pending_exceptions(self) {}

    pub fn move_reg_to_st0(&mut self, reg: Register) {
        match reg {
            Register::ST0 => self.move_to_st0(0),
            Register::ST1 => self.move_to_st0(1),
            Register::ST2 => self.move_to_st0(2),
            Register::ST3 => self.move_to_st0(3),
            Register::ST4 => self.move_to_st0(4),
            Register::ST5 => self.move_to_st0(5),
            Register::ST6 => self.move_to_st0(6),
            Register::ST7 => self.move_to_st0(7),
            _ => unimplemented!("impossible case"),
        }
    }

    pub fn reg_to_id(&self, reg: Register) -> usize {
        match reg {
            Register::ST0 => 0,
            Register::ST1 => 1,
            Register::ST2 => 2,
            Register::ST3 => 3,
            Register::ST4 => 4,
            Register::ST5 => 5,
            Register::ST6 => 6,
            Register::ST7 => 7,
            _ => unreachable!(),
        }
    }

    pub fn reg_to_idx(&self, reg: Register) -> usize {
        match reg {
            Register::ST0 => 0,
            Register::ST1 => 1,
            Register::ST2 => 2,
            Register::ST3 => 3,
            Register::ST4 => 4,
            Register::ST5 => 5,
            Register::ST6 => 6,
            Register::ST7 => 7,
            _ => unreachable!(),
        }
    }

    pub fn set_streg_f80(&mut self, reg: Register, value: u128) {
        //println!("{:?} {}", reg, value);
        let idx = self.reg_to_idx(reg);
        if let Some(st) = self.st.get_mut(idx) { st.set(value) }
    }

    pub fn set_streg(&mut self, reg: Register, value: f64) {
        //println!("{:?} {}", reg, value);
        let idx = self.reg_to_idx(reg);
        if let Some(st) = self.st.get_mut(idx) { st.set_f64(value) }
    }

    pub fn frexp(&self, value: f64) -> (f64, i32) {
        if value == 0.0 {
            (0.0, 0)
        } else {
            let exponent = value.abs().log2().floor() as i32 + 1;
            let mantissa = value / (2f64.powi(exponent));

            (mantissa, exponent)
        }
    }

    pub fn convert_st(&self, src: Vec<f64>) -> [u128; 8] {
        let mut result = [0u128; 8];

        for i in 0..8 {
            let low = if let Some(val) = src.get(i * 2) {
                *val
            } else {
                0.0
            };
            let high = if let Some(val) = src.get(i * 2 + 1) {
                *val
            } else {
                0.0
            };

            let low_bits = low.to_bits() as u128;
            let high_bits = high.to_bits() as u128;
            result[i] = low_bits | (high_bits << 64);
        }

        result
    }

    pub fn fxsave(&self) -> FPUState {
        let mut state = FPUState::new();
        state.fpu_control_word = self.fpu_control_word;
        state.fpu_status_word = self.stat;
        state.fpu_tag_word = self.tag;
        state.fpu_opcode = self.opcode;
        state.rip = self.ip;
        state.rdp = self.operand_ptr;
        state.mxcsr = self.mxcsr;
        state.mxcsr_mask = self.mxcsr;
        state.st = self.st;
        //state.st = self.convert_st(self.st.clone());
        state.xmm = self.xmm;
        state
    }

    pub fn fxrstor(&mut self, state: FPUState) {
        self.fpu_control_word = state.fpu_control_word;
        self.stat = state.fpu_status_word;
        self.tag = state.fpu_tag_word;
        self.opcode = state.fpu_opcode;
        self.ip = state.rip;
        self.operand_ptr = state.rdp;
        self.mxcsr = state.mxcsr;

        // Convert the packed 128-bit ST registers back to f64 values

        for i in 0..8 {
            if let Some(st) = self.st.get_mut(i) { st.fix() }
        }

        self.xmm = state.xmm;
    }
}
