use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetClassInfoExA(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExA");
}

fn get_class_info_ex(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api}");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

pub(super) fn GetClassInfoExW(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExW");
}
