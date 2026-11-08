use crate::emu;
use crate::winapi::helper;

pub fn InitializeAcl(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    log_red!(emu, "kernelbase!InitializeAcl p1:0x{:x} p2:0x{:x}", p1, p2);
    let handle = helper::handler_create("InitializeAcl");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
