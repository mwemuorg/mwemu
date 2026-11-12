use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHUpdateImageA(emu: &mut emu::Emu) {
    sh_update_image(emu, "SHUpdateImageA", false);
}

fn sh_update_image(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn SHUpdateImageW(emu: &mut emu::Emu) {
    sh_update_image(emu, "SHUpdateImageW", true);
}
