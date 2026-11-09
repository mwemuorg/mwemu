use crate::emu;

pub fn SetFileAttributesFromAppW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let attrs = emu.regs().rdx;
    let path = emu.maps.read_wide_string(path_ptr);
    log_red!(
        emu,
        "kernelbase!SetFileAttributesFromAppW '{}' attrs:0x{:x}",
        path,
        attrs
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
