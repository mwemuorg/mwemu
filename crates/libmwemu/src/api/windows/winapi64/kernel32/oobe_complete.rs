use crate::emu;

pub fn OOBEComplete(emu: &mut emu::Emu) {
    let p_is_complete = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!OOBEComplete pIsComplete:0x{:x}",
        p_is_complete
    );

    if p_is_complete != 0 {
        emu.maps.write_dword(p_is_complete, 1);
    }
    emu.regs_mut().rax = 1;
}
