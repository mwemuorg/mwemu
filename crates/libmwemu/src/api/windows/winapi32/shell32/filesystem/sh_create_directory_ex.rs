use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHCreateDirectoryExA(emu: &mut emu::Emu) {
    sh_create_directory_ex(emu, "SHCreateDirectoryExA", false);
}

pub fn SHCreateDirectoryExW(emu: &mut emu::Emu) {
    sh_create_directory_ex(emu, "SHCreateDirectoryExW", true);
}

fn sh_create_directory_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shcreatedirectoryexw");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    } else {
        let handle = helper::handler_create("shcreatedirectoryexa");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    }
}
