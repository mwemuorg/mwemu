use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHGetDiskFreeSpaceExA(emu: &mut emu::Emu) {
    sh_get_disk_free_space_ex(emu, "SHGetDiskFreeSpaceExA");
}

fn sh_get_disk_free_space_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn SHGetDiskFreeSpaceExW(emu: &mut emu::Emu) {
    sh_get_disk_free_space_ex(emu, "SHGetDiskFreeSpaceExW");
}
