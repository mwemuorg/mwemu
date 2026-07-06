use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexA(emu: &mut emu::Emu) {
    let attr = emu.regs().rcx;
    let owner = emu.regs().rdx;
    let name_ptr = emu.regs().r8;

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "kernel32!CreateMutexA '{}'", name);

    let uri = format!("mutex://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
