use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn Module32First(emu: &mut emu::Emu) {
    module32_first(emu, "Module32First");
}

fn module32_first(emu: &mut emu::Emu, api: &str) {
    let snap = WinApiAbi::arg(emu, 0) as u32;
    let entry = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "kernel32!{api} snap:0x{:x} entry:0x{:x}", snap, entry);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

pub fn Module32FirstW(emu: &mut emu::Emu) {
    module32_first(emu, "Module32FirstW");
}
