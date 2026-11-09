use crate::emu;

pub fn GetCurrentPackageInfo(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let buffer_length_ptr = emu.regs().rdx;
    let _buffer_ptr = emu.regs().r8;
    let count_ptr = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!GetCurrentPackageInfo flags:0x{:x} len_ptr:0x{:x}",
        flags,
        buffer_length_ptr
    );

    if buffer_length_ptr != 0 {
        emu.maps.write_dword(buffer_length_ptr, 0);
    }
    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
}
