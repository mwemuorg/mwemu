use crate::emu;

pub fn VerifyVersionInfoA_api(emu: &mut emu::Emu, api: &str) {
    let version_info = emu.regs().rcx;
    let type_mask = emu.regs().rdx;
    let condition_mask = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!{} info:0x{:x} type_mask:0x{:x} cond_mask:0x{:x} -> TRUE",
        api,
        version_info,
        type_mask,
        condition_mask
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
