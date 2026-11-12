use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHSetFolderPathA(emu: &mut emu::Emu) {
    sh_set_folder_path(emu, "SHSetFolderPathA");
}

fn sh_set_folder_path(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub fn SHSetFolderPathW(emu: &mut emu::Emu) {
    sh_set_folder_path(emu, "SHSetFolderPathW");
}
