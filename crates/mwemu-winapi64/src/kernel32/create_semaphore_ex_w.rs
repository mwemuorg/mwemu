use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreExW(emu: &mut emu::Emu) {
    let _security = emu.regs().rcx;
    let initial = emu.regs().rdx;
    let maximum = emu.regs().r8;
    let name = emu.regs().r9;
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!CreateSemaphoreExW init:{} max:{} name:'{}'",
        initial,
        maximum,
        name_s
    );
    emu.regs_mut().rax = helper::handler_create(&name_s);
}
