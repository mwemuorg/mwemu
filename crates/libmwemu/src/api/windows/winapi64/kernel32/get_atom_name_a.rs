use crate::emu;

pub fn GetAtomNameA(emu: &mut emu::Emu) {
    let atom = emu.regs().rcx as u16;
    let buf_ptr = emu.regs().rdx;
    let buf_size = emu.regs().r8 as u32;
    log_red!(
        emu,
        "kernel32!GetAtomNameA atom:0x{:x} ptr:0x{:x} size:{}",
        atom,
        buf_ptr,
        buf_size
    );
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_byte(buf_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
