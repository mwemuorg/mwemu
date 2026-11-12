use crate::emu;

pub fn SHEmptyRecycleBinA(emu: &mut emu::Emu) {
    sh_empty_recycle_bin(emu, "SHEmptyRecycleBinA");
}

fn sh_empty_recycle_bin(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHEmptyRecycleBinW(emu: &mut emu::Emu) {
    sh_empty_recycle_bin(emu, "SHEmptyRecycleBinW");
}
