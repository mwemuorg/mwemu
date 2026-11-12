use crate::emu;
use crate::winapi::helper;

pub fn Shell_GetCachedImageIndex(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!Shell_GetCachedImageIndex arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shell_getcachedimageindex");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn Shell_GetCachedImageIndexA(emu: &mut emu::Emu) {
    shell_get_cached_image_index(emu, "Shell_GetCachedImageIndexA", false);
}

pub fn Shell_GetCachedImageIndexW(emu: &mut emu::Emu) {
    shell_get_cached_image_index(emu, "Shell_GetCachedImageIndexW", true);
}

fn shell_get_cached_image_index(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shell_getcachedimageindexw");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    } else {
        let handle = helper::handler_create("shell_getcachedimageindexa");
        emu.set_last_error(0);
        emu.regs_mut().rax = handle;
    }
}
