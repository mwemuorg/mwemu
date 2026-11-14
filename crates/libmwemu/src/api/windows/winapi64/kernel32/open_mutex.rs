use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    open_mutex(emu, "OpenMutexA", false);
}

pub fn OpenMutexW(emu: &mut emu::Emu) {
    open_mutex(emu, "OpenMutexW", true);
}

fn open_mutex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let access = emu.regs().rcx;
    let inherit = emu.regs().rdx;
    let name = emu.regs().r8;
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    if wide {
        let hndl = helper::handler_create(&format!("mutex://{}", name_s));
        log_red!(
            emu,
            "kernel32!{api} access:0x{:x} name:'{}' => 0x{:x}",
            access,
            name_s,
            hndl
        );
        emu.regs_mut().rax = hndl;
        emu.set_last_error(0);
    } else {
        log_red!(
            emu,
            "kernel32!{api} access:0x{:x} name:'{}' => 0",
            access,
            name_s
        );
        emu.regs_mut().rax = 0;
        emu.set_last_error(2);
    }
}
