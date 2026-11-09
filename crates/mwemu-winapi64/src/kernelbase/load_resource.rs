use crate::emu;
use crate::winapi::helper;

pub fn LoadResource(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let hndl = helper::handler_create("resource_data");
    log_red!(emu, "kernelbase!LoadResource p1:0x{:x} => 0x{:x}", p1, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
