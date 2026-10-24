use crate::emu;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    let filename_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let filename = emu.maps.read_string(filename_ptr);

    log_red!(emu, "kernel32!GetFileAttributesA file: {}", filename);

    emu.regs_mut().rax = 0x123; // file attributes

    emu.stack_pop32(false);
}
