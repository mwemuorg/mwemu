use crate::emu;

pub fn GetCurrentPackagePath(emu: &mut emu::Emu) {
    let path_length_ptr = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetCurrentPackagePath length_ptr:0x{:x} path_ptr:0x{:x}",
        path_length_ptr,
        path_ptr
    );
    if path_length_ptr != 0 {
        emu.maps.write_dword(path_length_ptr, 0);
    }
    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
}
