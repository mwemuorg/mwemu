use crate::emu;
use crate::winapi::common::strings;

pub fn SendMessageTimeoutA(emu: &mut emu::Emu) {
    send_message_timeout(emu, "SendMessageTimeoutA", false);
}

fn send_message_timeout(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "user32!{api} '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SendMessageTimeoutW(emu: &mut emu::Emu) {
    send_message_timeout(emu, "SendMessageTimeoutW", true);
}
