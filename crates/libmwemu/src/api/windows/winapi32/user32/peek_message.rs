use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn PeekMessageA(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageA");
}

fn peek_message(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api}");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

pub(super) fn PeekMessageW(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageW");
}
