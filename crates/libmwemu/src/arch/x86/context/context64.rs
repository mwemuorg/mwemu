use crate::fpu::FPU;
use crate::maps::Maps;
use crate::regs64::Regs64;
use crate::windows::constants::exceptions::{CONTEXT_CONTROL, CONTEXT_INTEGER};

// https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-context

pub struct Context64 {
    p1_home: u64,
    p2_home: u64,
    p3_home: u64,
    p4_home: u64,
    p5_home: u64,
    p6_home: u64,
    ctx_flags: u32,
    mx_csr: u16,
    seg_cs: u16,
    seg_ds: u16,
    seg_es: u16,
    seg_fs: u16,
    seg_gs: u16,
    seg_ss: u16,
    eflags: u32,
    dr0: u64,
    dr1: u64,
    dr2: u64,
    dr3: u64,
    dr6: u64,
    dr7: u64,
    rax: u64,
    rcx: u64,
    rdx: u64,
    rbx: u64,
    rsp: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    fpu: FPU,
}

impl Context64 {
    const RAX_OFFSET: u64 = 120;
    const RCX_OFFSET: u64 = 128;
    const RDX_OFFSET: u64 = 136;
    const RBX_OFFSET: u64 = 144;
    const RBP_OFFSET: u64 = 160;
    const RSI_OFFSET: u64 = 168;
    const RDI_OFFSET: u64 = 176;
    const R8_OFFSET: u64 = 184;
    const R9_OFFSET: u64 = 192;
    const R10_OFFSET: u64 = 200;
    const R11_OFFSET: u64 = 208;
    const R12_OFFSET: u64 = 216;
    const R13_OFFSET: u64 = 224;
    const R14_OFFSET: u64 = 232;
    const R15_OFFSET: u64 = 240;
    const RSP_OFFSET: u64 = 152;
    const RIP_OFFSET: u64 = 248;
    const DR0_OFFSET: u64 = 72;
    const DR1_OFFSET: u64 = 80;
    const DR2_OFFSET: u64 = 88;
    const DR3_OFFSET: u64 = 96;
    const DR6_OFFSET: u64 = 104;
    const DR7_OFFSET: u64 = 112;
    pub const INTEGER_CONTROL_SIZE: u64 = 0x100;

    /// Transfer only the supported register groups selected by ContextFlags.
    pub(crate) fn load_selected(&mut self, addr: u64, maps: &Maps, flags: u32) {
        if flags & CONTEXT_INTEGER == CONTEXT_INTEGER {
            self.rax = maps.read_qword(addr + Self::RAX_OFFSET).unwrap_or(0);
            self.rcx = maps.read_qword(addr + Self::RCX_OFFSET).unwrap_or(0);
            self.rdx = maps.read_qword(addr + Self::RDX_OFFSET).unwrap_or(0);
            self.rbx = maps.read_qword(addr + Self::RBX_OFFSET).unwrap_or(0);
            self.rbp = maps.read_qword(addr + Self::RBP_OFFSET).unwrap_or(0);
            self.rsi = maps.read_qword(addr + Self::RSI_OFFSET).unwrap_or(0);
            self.rdi = maps.read_qword(addr + Self::RDI_OFFSET).unwrap_or(0);
            self.r8 = maps.read_qword(addr + Self::R8_OFFSET).unwrap_or(0);
            self.r9 = maps.read_qword(addr + Self::R9_OFFSET).unwrap_or(0);
            self.r10 = maps.read_qword(addr + Self::R10_OFFSET).unwrap_or(0);
            self.r11 = maps.read_qword(addr + Self::R11_OFFSET).unwrap_or(0);
            self.r12 = maps.read_qword(addr + Self::R12_OFFSET).unwrap_or(0);
            self.r13 = maps.read_qword(addr + Self::R13_OFFSET).unwrap_or(0);
            self.r14 = maps.read_qword(addr + Self::R14_OFFSET).unwrap_or(0);
            self.r15 = maps.read_qword(addr + Self::R15_OFFSET).unwrap_or(0);
        }
        if flags & CONTEXT_CONTROL == CONTEXT_CONTROL {
            self.rsp = maps.read_qword(addr + Self::RSP_OFFSET).unwrap_or(0);
            self.rip = maps.read_qword(addr + Self::RIP_OFFSET).unwrap_or(0);
            self.eflags = maps.read_dword(addr + Self::EFLAGS_OFFSET).unwrap_or(0);
        }
    }

    fn save_registers(&self, addr: u64, maps: &mut Maps, flags: u32) {
        if flags & CONTEXT_INTEGER == CONTEXT_INTEGER {
            maps.write_qword(addr + Self::RAX_OFFSET, self.rax);
            maps.write_qword(addr + Self::RCX_OFFSET, self.rcx);
            maps.write_qword(addr + Self::RDX_OFFSET, self.rdx);
            maps.write_qword(addr + Self::RBX_OFFSET, self.rbx);
            maps.write_qword(addr + Self::RBP_OFFSET, self.rbp);
            maps.write_qword(addr + Self::RSI_OFFSET, self.rsi);
            maps.write_qword(addr + Self::RDI_OFFSET, self.rdi);
            maps.write_qword(addr + Self::R8_OFFSET, self.r8);
            maps.write_qword(addr + Self::R9_OFFSET, self.r9);
            maps.write_qword(addr + Self::R10_OFFSET, self.r10);
            maps.write_qword(addr + Self::R11_OFFSET, self.r11);
            maps.write_qword(addr + Self::R12_OFFSET, self.r12);
            maps.write_qword(addr + Self::R13_OFFSET, self.r13);
            maps.write_qword(addr + Self::R14_OFFSET, self.r14);
            maps.write_qword(addr + Self::R15_OFFSET, self.r15);
        }
        if flags & CONTEXT_CONTROL == CONTEXT_CONTROL {
            maps.write_qword(addr + Self::RSP_OFFSET, self.rsp);
            maps.write_qword(addr + Self::RIP_OFFSET, self.rip);
        }
    }

    pub(crate) fn save_selected(&self, addr: u64, maps: &mut Maps, flags: u32, eflags: u32) {
        self.save_registers(addr, maps, flags);
        if flags & CONTEXT_CONTROL == CONTEXT_CONTROL {
            maps.write_dword(addr + Self::EFLAGS_OFFSET, eflags);
        }
    }

    pub const FLAGS_OFFSET: u64 = 0x30;
    pub const EFLAGS_OFFSET: u64 = 0x44;

    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    pub fn new(regs: &Regs64) -> Context64 {
        Context64 {
            p1_home: 0,
            p2_home: 0,
            p3_home: 0,
            p4_home: 0,
            p5_home: 0,
            p6_home: 0,
            ctx_flags: 0,
            dr0: 0,
            dr1: 0,
            dr2: 0,
            dr3: 0,
            dr6: 0,
            dr7: 0,
            mx_csr: 0,
            seg_fs: 0,
            seg_gs: 0,
            seg_es: 0,
            seg_ds: 0,
            seg_cs: 0,
            seg_ss: 0,
            eflags: 0,
            rdi: regs.rdi,
            rsi: regs.rsi,
            rbx: regs.rbx,
            rdx: regs.rdx,
            rcx: regs.rcx,
            rax: regs.rax,
            rbp: regs.rbp,
            rip: regs.rip,
            rsp: regs.rsp,
            r8: regs.r8,
            r9: regs.r9,
            r10: regs.r10,
            r11: regs.r11,
            r12: regs.r12,
            r13: regs.r13,
            r14: regs.r14,
            r15: regs.r15,
            fpu: FPU::new(),
        }
    }

    /*
    typedef struct _CONTEXT {
      DWORD64 P1Home; 0
      DWORD64 P2Home; 8
      DWORD64 P3Home; 16
      DWORD64 P4Home; 24
      DWORD64 P5Home; 32
      DWORD64 P6Home; 40
      DWORD   ContextFlags; 48
      DWORD   MxCsr; 52
      WORD    SegCs; 56
      WORD    SegDs; 58
      WORD    SegEs; 60
      WORD    SegFs; 62
      WORD    SegGs; 64
      WORD    SegSs; 66
      DWORD   EFlags; 68
      DWORD64 Dr0; 72
      DWORD64 Dr1; 80
      DWORD64 Dr2; 88
      DWORD64 Dr3; 96
      DWORD64 Dr6; 104
      DWORD64 Dr7; 112
      DWORD64 Rax; 120
      DWORD64 Rcx; 128
      DWORD64 Rdx; 136
      DWORD64 Rbx; 144
      DWORD64 Rsp; 152
      DWORD64 Rbp; 160
      DWORD64 Rsi; 168
      DWORD64 Rdi; 176
      DWORD64 R8; 184
      DWORD64 R9; 192
      DWORD64 R10; 200
      DWORD64 R11; 208
      DWORD64 R12; 216
      DWORD64 R13; 224
      DWORD64 R14; 232
      DWORD64 R15; 240
      DWORD64 Rip; 248
        */

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr + Self::DR0_OFFSET, self.dr0);
        maps.write_qword(addr + Self::DR1_OFFSET, self.dr1);
        maps.write_qword(addr + Self::DR2_OFFSET, self.dr2);
        maps.write_qword(addr + Self::DR3_OFFSET, self.dr3);
        maps.write_qword(addr + Self::DR6_OFFSET, self.dr6);
        maps.write_qword(addr + Self::DR7_OFFSET, self.dr7);

        self.save_registers(addr, maps, CONTEXT_CONTROL | CONTEXT_INTEGER);
    }

    pub fn load(&mut self, addr: u64, maps: &mut Maps) {
        self.dr0 = maps.read_qword(addr + Self::DR0_OFFSET).unwrap_or(0);
        self.dr1 = maps.read_qword(addr + Self::DR1_OFFSET).unwrap_or(0);
        self.dr2 = maps.read_qword(addr + Self::DR2_OFFSET).unwrap_or(0);
        self.dr3 = maps.read_qword(addr + Self::DR3_OFFSET).unwrap_or(0);
        self.dr6 = maps.read_qword(addr + Self::DR6_OFFSET).unwrap_or(0);
        self.dr7 = maps.read_qword(addr + Self::DR7_OFFSET).unwrap_or(0);

        self.load_selected(addr, maps, CONTEXT_CONTROL | CONTEXT_INTEGER);
    }

    pub fn sync(&self, regs: &mut Regs64) {
        regs.rax = self.rax;
        regs.rbx = self.rbx;
        regs.rcx = self.rcx;
        regs.rdx = self.rdx;
        regs.rsi = self.rsi;
        regs.rdi = self.rdi;
        regs.rsp = self.rsp;
        regs.rbp = self.rbp;
        regs.rip = self.rip;

        regs.r8 = self.r8;
        regs.r9 = self.r9;
        regs.r10 = self.r10;
        regs.r11 = self.r11;
        regs.r12 = self.r12;
        regs.r13 = self.r13;
        regs.r14 = self.r14;
        regs.r15 = self.r15;
    }
}
