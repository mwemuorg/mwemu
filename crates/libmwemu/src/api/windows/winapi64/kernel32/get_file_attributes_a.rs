use crate::emu;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;
    let filename = emu.maps.read_string(filename_ptr);

    log_red!(emu, "kernel32!GetFileAttributesA file: {}", filename);
    emu.regs_mut().rax = 0x123;
}
