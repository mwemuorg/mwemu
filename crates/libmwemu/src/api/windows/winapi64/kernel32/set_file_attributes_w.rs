use crate::emu;

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs().rcx);
    let attrs = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!SetFileAttributesW '{}' attrs:0x{:x}",
        path,
        attrs
    );
    emu.regs_mut().rax = 1;
}
