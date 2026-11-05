use crate::emu;

pub fn SdbpGetPathCustomSdb(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;

    log_red!(emu, "kernel32!SdbpGetPathCustomSdb buffer:0x{:x}", buffer);

    if buffer != 0 {
        emu.maps.write_wide_string(buffer, "C:\\Windows\\AppPatch");
    }
    emu.regs_mut().rax = 1;
}
