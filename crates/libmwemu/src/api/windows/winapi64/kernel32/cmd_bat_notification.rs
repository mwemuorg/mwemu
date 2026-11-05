use crate::emu;

pub fn CmdBatNotification(emu: &mut emu::Emu) {
    let notification_type = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CmdBatNotification type:0x{:x}",
        notification_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
