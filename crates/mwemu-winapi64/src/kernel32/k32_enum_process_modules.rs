use crate::emu;

pub fn K32EnumProcessModules(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!K32EnumProcessModules handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
