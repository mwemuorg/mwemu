use crate::emu;

pub fn GetCurrentPackageFamilyName(emu: &mut emu::Emu) {
    let length_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetCurrentPackageFamilyName length_ptr:0x{:x} name_ptr:0x{:x}",
        length_ptr,
        name_ptr
    );

    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 0);
    }

    emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
}
