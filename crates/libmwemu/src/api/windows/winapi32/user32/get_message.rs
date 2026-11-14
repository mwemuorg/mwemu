use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetMessageA(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageA");
}

fn get_message(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api}");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub(super) fn GetMessageW(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageW");
}
