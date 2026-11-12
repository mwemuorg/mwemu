use crate::emu;
use crate::winapi::common::strings;

pub fn GetWindowModuleFileName(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "user32!GetWindowModuleFileName h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetWindowModuleFileNameA(emu: &mut emu::Emu) {
    get_window_module_file_name(emu, "GetWindowModuleFileNameA", false);
}

fn get_window_module_file_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetWindowModuleFileNameW(emu: &mut emu::Emu) {
    get_window_module_file_name(emu, "GetWindowModuleFileNameW", true);
}
