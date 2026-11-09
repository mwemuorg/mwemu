use crate::emu;

pub fn SetStdHandle(emu: &mut emu::Emu) {
    let nstd = emu.regs().rcx as u32;
    let handle = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!SetStdHandle nstd:0x{:x} handle:0x{:x}",
        nstd,
        handle
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
