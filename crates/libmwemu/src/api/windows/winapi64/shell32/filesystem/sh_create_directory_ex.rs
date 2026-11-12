use crate::emu;
use crate::winapi::helper;

pub fn SHCreateDirectoryExA(emu: &mut emu::Emu) {
    sh_create_directory_ex(emu, "SHCreateDirectoryExA", false);
}

pub fn SHCreateDirectoryExW(emu: &mut emu::Emu) {
    sh_create_directory_ex(emu, "SHCreateDirectoryExW", true);
}

fn sh_create_directory_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shcreatedirectoryexw");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    } else {
        let handle = helper::handler_create("shcreatedirectoryexa");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    }
}
