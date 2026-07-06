use crate::emu;

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let filename = emu.maps.read_wide_string(filename_ptr);

    log_red!(emu, "kernel32!GetFileAttributesW file: {}", filename);
    emu.regs_mut().rax = 0x123;
}
