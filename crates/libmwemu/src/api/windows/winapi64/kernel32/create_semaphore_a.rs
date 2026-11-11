use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreA(emu: &mut emu::Emu, api: &str) {
    let _security = emu.regs().rcx;
    let initial = emu.regs().rdx;
    let maximum = emu.regs().r8;
    let name = emu.regs().r9;
    let name_s = if name != 0 {
        emu.maps.read_string(name)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!{} init:{} max:{} name:'{}'",
        api,
        initial,
        maximum,
        name_s
    );
    emu.regs_mut().rax = helper::handler_create(&name_s);
}
