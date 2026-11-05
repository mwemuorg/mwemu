use crate::emu;

pub fn FormatApplicationUserModelId(emu: &mut emu::Emu) {
    let p_family_name = emu.regs().rcx;
    let p_app_id = emu.regs().rdx;
    let p_app_model_id_len = emu.regs().r8;
    let p_app_model_id = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!FormatApplicationUserModelId family:0x{:x} appId:0x{:x}",
        p_family_name,
        p_app_id
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
