use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegisterClipboardFormatA(emu: &mut emu::Emu) {
    register_clipboard_format(emu, "RegisterClipboardFormatA");
}

fn register_clipboard_format(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xc000;
    stdcall_ret(emu, 1);
}

pub fn RegisterClipboardFormatW(emu: &mut emu::Emu) {
    register_clipboard_format(emu, "RegisterClipboardFormatW");
}
