use crate::fpu::FPU;
use crate::maps::Maps;
use crate::regs64::Regs64;
use crate::windows::constants::exceptions::{WOW64_CONTEXT_CONTROL, WOW64_CONTEXT_INTEGER};

pub struct Context32 {
    ctx_flags: u32,
    dr0: u32,
    dr1: u32,
    dr2: u32,
    dr3: u32,
    dr6: u32,
    dr7: u32,
    fpu: FPU,
    seg_gs: u32,
    seg_fd: u32,
    seg_es: u32,
    seg_ds: u32,
    edi: u32,    // +9c
    esi: u32,    // +a0
    ebx: u32,    // +a4
    edx: u32,    // +a8
    ecx: u32,    // +ac
    eax: u32,    // +b0
    ebp: u32,    // +b4
    eip: u32,    // +b8
    seg_cs: u32, // +bc
    eflags: u32, // +c0
    esp: u32,    // +c4
    seg_ss: u32,
}

impl Context32 {
    const EDI_OFFSET: u64 = 0x9c;
    const ESI_OFFSET: u64 = 0xa0;
    const EBX_OFFSET: u64 = 0xa4;
    const EDX_OFFSET: u64 = 0xa8;
    const ECX_OFFSET: u64 = 0xac;
    const EAX_OFFSET: u64 = 0xb0;
    const EBP_OFFSET: u64 = 0xb4;
    const EIP_OFFSET: u64 = 0xb8;
    const ESP_OFFSET: u64 = 0xc4;
    const DR0_OFFSET: u64 = 4;
    const DR1_OFFSET: u64 = 8;
    const DR2_OFFSET: u64 = 12;
    const DR3_OFFSET: u64 = 16;
    const DR6_OFFSET: u64 = 20;
    const DR7_OFFSET: u64 = 24;
    pub const INTEGER_CONTROL_SIZE: u64 = 0xcc;

    /// Transfer only the supported register groups selected by ContextFlags.
    pub(crate) fn load_selected(&mut self, addr: u64, maps: &Maps, flags: u32) {
        if flags & WOW64_CONTEXT_INTEGER == WOW64_CONTEXT_INTEGER {
            self.edi = maps.read_dword(addr + Self::EDI_OFFSET).unwrap_or(0);
            self.esi = maps.read_dword(addr + Self::ESI_OFFSET).unwrap_or(0);
            self.ebx = maps.read_dword(addr + Self::EBX_OFFSET).unwrap_or(0);
            self.edx = maps.read_dword(addr + Self::EDX_OFFSET).unwrap_or(0);
            self.ecx = maps.read_dword(addr + Self::ECX_OFFSET).unwrap_or(0);
            self.eax = maps.read_dword(addr + Self::EAX_OFFSET).unwrap_or(0);
        }
        if flags & WOW64_CONTEXT_CONTROL == WOW64_CONTEXT_CONTROL {
            self.ebp = maps.read_dword(addr + Self::EBP_OFFSET).unwrap_or(0);
            self.eip = maps.read_dword(addr + Self::EIP_OFFSET).unwrap_or(0);
            self.esp = maps.read_dword(addr + Self::ESP_OFFSET).unwrap_or(0);
            self.eflags = maps.read_dword(addr + Self::EFLAGS_OFFSET).unwrap_or(0);
        }
    }

    fn save_registers(&self, addr: u64, maps: &mut Maps, flags: u32) {
        if flags & WOW64_CONTEXT_INTEGER == WOW64_CONTEXT_INTEGER {
            maps.write_dword(addr + Self::EDI_OFFSET, self.edi);
            maps.write_dword(addr + Self::ESI_OFFSET, self.esi);
            maps.write_dword(addr + Self::EBX_OFFSET, self.ebx);
            maps.write_dword(addr + Self::EDX_OFFSET, self.edx);
            maps.write_dword(addr + Self::ECX_OFFSET, self.ecx);
            maps.write_dword(addr + Self::EAX_OFFSET, self.eax);
        }
        if flags & WOW64_CONTEXT_CONTROL == WOW64_CONTEXT_CONTROL {
            maps.write_dword(addr + Self::EBP_OFFSET, self.ebp);
            maps.write_dword(addr + Self::EIP_OFFSET, self.eip);
            maps.write_dword(addr + Self::ESP_OFFSET, self.esp);
        }
    }

    pub(crate) fn save_selected(&self, addr: u64, maps: &mut Maps, flags: u32, eflags: u32) {
        self.save_registers(addr, maps, flags);
        if flags & WOW64_CONTEXT_CONTROL == WOW64_CONTEXT_CONTROL {
            maps.write_dword(addr + Self::EFLAGS_OFFSET, eflags);
        }
    }

    pub const FLAGS_OFFSET: u64 = 0x0;
    pub const EFLAGS_OFFSET: u64 = 0xc0;

    pub fn new(regs: &Regs64) -> Context32 {
        Context32 {
            ctx_flags: 0,
            dr0: 0,
            dr1: 0,
            dr2: 0,
            dr3: 0,
            dr6: 0,
            dr7: 0,
            fpu: FPU::new(),
            seg_gs: 0,
            seg_fd: 0,
            seg_es: 0,
            seg_ds: 0,
            edi: regs.get_edi() as u32,
            esi: regs.get_esi() as u32,
            ebx: regs.get_ebx() as u32,
            edx: regs.get_edx() as u32,
            ecx: regs.get_ecx() as u32,
            eax: regs.get_eax() as u32,
            ebp: regs.get_ebp() as u32,
            eip: regs.get_eip() as u32,
            seg_cs: 0,
            eflags: 0,
            esp: regs.get_esp() as u32,
            seg_ss: 0,
        }
    }

    pub fn save(&self, addr: u32, maps: &mut Maps) {
        let addr = addr as u64;
        maps.write_dword(addr + Self::DR0_OFFSET, self.dr0);
        maps.write_dword(addr + Self::DR1_OFFSET, self.dr1);
        maps.write_dword(addr + Self::DR2_OFFSET, self.dr2);
        maps.write_dword(addr + Self::DR3_OFFSET, self.dr3);
        maps.write_dword(addr + Self::DR6_OFFSET, self.dr6);
        maps.write_dword(addr + Self::DR7_OFFSET, self.dr7);

        self.save_registers(addr, maps, WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER);
    }

    pub fn load(&mut self, addr: u32, maps: &mut Maps) {
        let addr = addr as u64;
        self.dr0 = maps.read_dword(addr + Self::DR0_OFFSET).unwrap_or(0);
        self.dr1 = maps.read_dword(addr + Self::DR1_OFFSET).unwrap_or(0);
        self.dr2 = maps.read_dword(addr + Self::DR2_OFFSET).unwrap_or(0);
        self.dr3 = maps.read_dword(addr + Self::DR3_OFFSET).unwrap_or(0);
        self.dr6 = maps.read_dword(addr + Self::DR6_OFFSET).unwrap_or(0);
        self.dr7 = maps.read_dword(addr + Self::DR7_OFFSET).unwrap_or(0);

        self.load_selected(addr, maps, WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER);
    }

    pub fn sync(&self, regs: &mut Regs64) {
        regs.set_eax(self.eax as u64);
        regs.set_ebx(self.ebx as u64);
        regs.set_ecx(self.ecx as u64);
        regs.set_edx(self.edx as u64);
        regs.set_esi(self.esi as u64);
        regs.set_edi(self.edi as u64);
        regs.set_esp(self.esp as u64);
        regs.set_ebp(self.ebp as u64);
        regs.set_eip(self.eip as u64);
    }
}
