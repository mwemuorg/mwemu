use crate::emu;

pub fn SetDefaultDllDirectories(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!SetDefaultDllDirectories flags:0x{:x}",
        flags
    );
    let _ = emu.maps.is_mapped(emu.regs().rcx);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
