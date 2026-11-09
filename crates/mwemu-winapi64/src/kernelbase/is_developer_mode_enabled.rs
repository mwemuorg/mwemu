use crate::emu;
use crate::winapi::helper;

pub fn IsDeveloperModeEnabled(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!IsDeveloperModeEnabled 0x{:x} => TRUE",
        arg1
    );
    let handle = helper::handler_create("IsDeveloperModeEnabled");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
