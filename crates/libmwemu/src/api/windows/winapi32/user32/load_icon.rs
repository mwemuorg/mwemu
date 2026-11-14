use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn LoadIconA(emu: &mut emu::Emu) {
    let hinstance = arg(emu, 0);
    let name = arg(emu, 1);
    log_red!(
        emu,
        "user32!LoadIconA hInstance: 0x{:x} name: 0x{:x}",
        hinstance,
        name
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

pub fn LoadIconW(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "user32!LoadIconW arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.regs_mut().rax = helper::handler_create("loadiconw");
    stdcall_ret(emu, 2);
}
