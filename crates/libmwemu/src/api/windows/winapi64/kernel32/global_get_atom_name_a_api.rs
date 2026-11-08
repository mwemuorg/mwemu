use crate::emu;

pub fn GlobalGetAtomNameA_api(emu: &mut emu::Emu) {
    let atom = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let name = format!("Atom#{:04X}", atom);
    log_red!(
        emu,
        "kernel32!GlobalGetAtomNameA atom:0x{:x} => '{}'",
        atom,
        name
    );
    if buf != 0 && buf_size > 0 {
        emu.maps.write_string(buf, &name);
        emu.regs_mut().rax = name.len() as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
