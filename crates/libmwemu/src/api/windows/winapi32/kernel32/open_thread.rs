use crate::emu;
use crate::winapi::helper;

pub fn OpenThread(emu: &mut emu::Emu) {
    let access = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let inherit = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let tid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!OpenThread tid: {}", tid);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let uri = format!("tid://{}", tid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
