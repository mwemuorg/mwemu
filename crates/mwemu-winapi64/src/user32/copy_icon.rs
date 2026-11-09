use crate::emu;
use crate::winapi::helper;

pub fn CopyIcon(emu: &mut emu::Emu) {
    let hicon = emu.regs().rcx;
    log_red!(emu, "user32!CopyIcon hIcon:0x{:x}", hicon);
    let hndl = helper::handler_create("user32://icon");
    emu.regs_mut().rax = hndl;
}
