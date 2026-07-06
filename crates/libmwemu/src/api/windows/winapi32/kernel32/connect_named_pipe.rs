use crate::emu;
use crate::winapi::helper;

pub fn ConnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ConnectNamedPipe cannot read the handle") as u64;
    let overlapped = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!ConnectNamedPipe cannot read the overlapped");

    log_red!(emu, "kernel32!ConnectNamedPipe hndl: 0x{:x}", handle);
    if !helper::handler_exist(handle) {
        log::trace!("\tinvalid handle.");
    }

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
