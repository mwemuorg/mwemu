use crate::emu;
use crate::winapi::helper;

pub fn FindClose(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FindClose cannot read the handle") as u64;

    log_red!(emu, "kernel32!FindClose");

    emu.stack_pop32(false);

    helper::handler_close(hndl);
    emu.regs_mut().rax = 1;
}
