use crate::emu;

pub fn ClearCommError(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_errors = emu.regs().rdx;
    let lp_stat = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!ClearCommError hFile:0x{:x} lpErrors:0x{:x} lpStat:0x{:x}",
        h_file,
        lp_errors,
        lp_stat
    );

    if lp_errors != 0 {
        emu.maps.write_dword(lp_errors, 0);
    }
    emu.regs_mut().rax = 1;
}
