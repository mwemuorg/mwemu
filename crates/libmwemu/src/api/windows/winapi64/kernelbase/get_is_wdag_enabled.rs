use crate::emu;
use crate::winapi::helper;

pub fn GetIsWdagEnabled(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!GetIsWdagEnabled h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let handle = helper::handler_create("GetIsWdagEnabled");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
