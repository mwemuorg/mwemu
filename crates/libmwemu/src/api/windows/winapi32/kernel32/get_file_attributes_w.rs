use crate::emu;

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    let filename_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!GetFileAttributesW cannot read filename_ptr") as u64;
    let filename = emu.maps.read_wide_string(filename_ptr);

    log_red!(emu, "kernel32!GetFileAttributesW file: {}", filename);

    emu.stack_pop32(false);

    emu.regs_mut().rax = 0x123; // file attributes
}
