use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn OleSetClipboard(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "ole32!OleSetClipboard a0:0x{:x}", _a0);
    let _h = helper::handler_create("ole32://OleSetClipboard");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
