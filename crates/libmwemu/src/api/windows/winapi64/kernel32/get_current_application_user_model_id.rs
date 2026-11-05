use crate::emu;

pub fn GetCurrentApplicationUserModelId(emu: &mut emu::Emu) {
    let length_ptr = emu.regs().rcx;
    let app_id_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetCurrentApplicationUserModelId length_ptr:0x{:x} id_ptr:0x{:x}",
        length_ptr,
        app_id_ptr
    );

    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
}
