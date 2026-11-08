use crate::emu;
use crate::winapi::helper;

pub fn ConnectNamedPipe(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    log_red!(emu, "kernelbase!ConnectNamedPipe p1:0x{:x}", p1);
    let handle = helper::handler_create("ConnectNamedPipe");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
