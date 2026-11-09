use crate::emu;

pub fn SetFileAttributesA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let attrs = emu.regs().rdx;
    let path = emu.maps.read_string(path_ptr);
    log_red!(
        emu,
        "kernelbase!SetFileAttributesA '{}' attrs:0x{:x}",
        path,
        attrs
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
