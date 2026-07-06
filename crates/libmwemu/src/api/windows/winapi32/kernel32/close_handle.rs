use crate::emu;
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CloseHandle cannot read the handle") as u64;

    log_red!(emu, "kernel32!CloseHandle 0x{:X}", hndl);

    if !helper::handler_close(hndl) {
        log::trace!("\tinvalid handle.")
    }
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
