use crate::emu;

pub fn GetOEMCP(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let out_buf = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!GetOEMCP {}:0x{:x} {}:0x{:x}",
        "handle",
        handle,
        "out_buf",
        out_buf
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
