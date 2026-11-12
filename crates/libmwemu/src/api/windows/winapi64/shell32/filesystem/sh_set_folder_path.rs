use crate::emu;

pub fn SHSetFolderPathA(emu: &mut emu::Emu) {
    sh_set_folder_path(emu, "SHSetFolderPathA");
}

fn sh_set_folder_path(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn SHSetFolderPathW(emu: &mut emu::Emu) {
    sh_set_folder_path(emu, "SHSetFolderPathW");
}
