use crate::emu;

pub fn GetConsoleMode(emu: &mut emu::Emu) {
    let h_console_handle = emu.regs().rcx;
    let lp_mode = emu.regs().rdx as usize;
    log_red!(
        emu,
        "** {} kernel32!GetConsoleMode {:x} {:x}",
        emu.pos,
        h_console_handle,
        lp_mode
    );
    // TODO: implement this
    emu.maps.write_dword(lp_mode as u64, 0x00000003); //TODO: not sure what this is
    emu.regs_mut().rax = 1;
}
