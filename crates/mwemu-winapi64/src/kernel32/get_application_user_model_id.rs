use crate::emu;

pub fn GetApplicationUserModelId(emu: &mut emu::Emu) {
    let process = emu.regs().rcx;
    let length_ptr = emu.regs().rdx;
    let app_id_ptr = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!GetApplicationUserModelId proc:0x{:x} length_ptr:0x{:x}",
        process,
        length_ptr
    );

    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
}
