use crate::emu;

pub fn GlobalGetAtomNameW(emu: &mut emu::Emu) {
    let atom = emu.regs().rcx as u16;
    let lp_buffer = emu.regs().rdx;
    let n_size = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!GlobalGetAtomNameW atom:0x{:x} buf:0x{:x} size:{}",
        atom,
        lp_buffer,
        n_size
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    if lp_buffer != 0 && n_size > 4 {
        emu.maps.write_wide_string(lp_buffer, "ATOM");
        emu.regs_mut().rax = 4;
    } else {
        emu.regs_mut().rax = 0;
    }
}
