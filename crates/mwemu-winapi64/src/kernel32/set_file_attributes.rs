use crate::emu;

pub fn SetFileAttributesA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rcx);
    let attrs = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!SetFileAttributesA '{}' attrs:0x{:x}",
        path,
        attrs
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs().rcx);
    let attrs = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!SetFileAttributesW '{}' attrs:0x{:x}",
        path,
        attrs
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
