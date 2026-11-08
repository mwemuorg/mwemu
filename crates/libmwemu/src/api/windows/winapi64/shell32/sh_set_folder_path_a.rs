use crate::emu;

pub fn SHSetFolderPathA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx as u32;
    log_red!(emu, "shell32!SHSetFolderPathA arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
