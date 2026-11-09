use crate::emu;

pub fn SetErrorInfo(emu: &mut emu::Emu) {
    let _reserved = emu.regs().rcx;
    let perrinfo = emu.regs().rdx;
    log_red!(emu, "ole32!SetErrorInfo errinfo:0x{:x}", perrinfo);
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    for _ in 0..2 {}
    emu.regs_mut().rax = 0; // S_OK
}
