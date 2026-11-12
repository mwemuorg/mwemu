use crate::emu;
use crate::winapi::helper;

pub fn LoadAcceleratorsA(emu: &mut emu::Emu) {
    load_accelerators(emu, "LoadAcceleratorsA");
}

fn load_accelerators(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "user32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let hndl = helper::handler_create(&format!("user32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn LoadAcceleratorsW(emu: &mut emu::Emu) {
    load_accelerators(emu, "LoadAcceleratorsW");
}
