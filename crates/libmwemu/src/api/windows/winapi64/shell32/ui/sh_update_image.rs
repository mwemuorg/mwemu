use crate::emu;

pub fn SHUpdateImageA(emu: &mut emu::Emu) {
    sh_update_image(emu, "SHUpdateImageA");
}

fn sh_update_image(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHUpdateImageW(emu: &mut emu::Emu) {
    sh_update_image(emu, "SHUpdateImageW");
}
