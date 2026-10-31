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
    emu.regs_mut().rax = 1;
}
