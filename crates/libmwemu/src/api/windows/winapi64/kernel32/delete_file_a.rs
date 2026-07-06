use crate::emu;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;

    let filename = emu.maps.read_string(filename_ptr);

    log_red!(
        emu,
        "kernel32!DeleteFileA `{}` (stub: not deleting)",
        filename
    );

    emu.regs_mut().rax = 1; // Success
}
