use crate::emu;
use crate::winapi::helper;

pub fn OpenMutexW(emu: &mut emu::Emu) {
    let access = emu.regs().rcx;
    let inherit = emu.regs().rdx;
    let name = emu.regs().r8;
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    let hndl = helper::handler_create(&format!("mutex://{}", name_s));
    log_red!(
        emu,
        "kernel32!OpenMutexW access:0x{:x} name:'{}' => 0x{:x}",
        access,
        name_s,
        hndl
    );
    emu.regs_mut().rax = hndl;
    emu.set_last_error(0);
}
