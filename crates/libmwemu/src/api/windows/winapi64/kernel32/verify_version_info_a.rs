use crate::emu;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    let version_info = emu.regs().rcx;
    let type_mask = emu.regs().rdx;
    let condition_mask = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!VerifyVersionInfoA pInfo:0x{:x} typeMask:0x{:x} condMask:0x{:x}",
        version_info,
        type_mask,
        condition_mask
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
