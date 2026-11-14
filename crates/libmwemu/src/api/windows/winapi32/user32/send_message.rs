use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SendMessageA(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageA");
}

fn send_message(emu: &mut emu::Emu, api: &str) {
    log_red!(
        emu,
        "user32!{api} hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

pub(super) fn SendMessageW(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageW");
}
