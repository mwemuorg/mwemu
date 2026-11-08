use crate::emu;
use crate::winapi::helper;
pub fn _TlgCreateWsz(emu: &mut emu::Emu) {
    let p_desc = emu.regs().rcx;
    let p_wsz = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!_TlgCreateWsz pDesc:0x{:x} pWsz:0x{:x}",
        p_desc,
        p_wsz
    );
    let handle = helper::handler_create("TlgCreateWsz");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
