use crate::emu;
use crate::winapi::common::strings;

pub fn GetClassNameA(emu: &mut emu::Emu) {
    get_class_name(emu, "GetClassNameA", false);
}

fn get_class_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    let class_name = "mwemu_class";
    if buf != 0 && max_count > 0 {
        strings::write_string(emu, buf, class_name, wide);
    }
    log_red!(emu, "user32!{api} hwnd:0x{:x} => '{}'", hwnd, class_name);

    emu.set_last_error(0);
    emu.regs_mut().rax = class_name.len() as u64;
}

pub fn GetClassNameW(emu: &mut emu::Emu) {
    get_class_name(emu, "GetClassNameW", true);
}
