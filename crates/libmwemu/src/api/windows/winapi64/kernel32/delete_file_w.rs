use crate::emu;

pub fn DeleteFileW(emu: &mut emu::Emu) {
    let filename_ptr = emu.regs().rcx;

    let filename = emu.maps.read_wide_string(filename_ptr);

    log_red!(
        emu,
        "kernel32!DeleteFileW `{}` (stub: not deleting)",
        filename
    );

    emu.regs_mut().rax = 1;
}
