use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn Shell_GetCachedImageIndex(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!Shell_GetCachedImageIndex arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shell_getcachedimageindex");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 3);
}

pub fn Shell_GetCachedImageIndexA(emu: &mut emu::Emu) {
    shell_get_cached_image_index(emu, "Shell_GetCachedImageIndexA", false);
}

pub fn Shell_GetCachedImageIndexW(emu: &mut emu::Emu) {
    shell_get_cached_image_index(emu, "Shell_GetCachedImageIndexW", true);
}

fn shell_get_cached_image_index(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shell_getcachedimageindexw");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    } else {
        let handle = helper::handler_create("shell_getcachedimageindexa");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 3);
    }
}
