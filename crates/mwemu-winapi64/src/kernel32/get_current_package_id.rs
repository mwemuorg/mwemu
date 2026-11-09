use crate::emu;

pub fn GetCurrentPackageId(emu: &mut emu::Emu) {
    let buffer_length_ptr = emu.regs().rcx;
    let _buffer_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetCurrentPackageId buf_len_ptr:0x{:x}",
        buffer_length_ptr
    );

    if buffer_length_ptr != 0 {
        emu.maps.write_dword(buffer_length_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
}
