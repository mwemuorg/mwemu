use super::allocate_memory;
use crate::emu;

pub(super) fn __p___argc(emu: &mut emu::Emu) {
    let argc = emu.regs().rcx;

    log_red!(emu, "wincrt!__p___argc argc: 0x{:x}", argc);

    let argc_addr = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argc cannot allocate");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    emu.maps.write_dword(argc_addr, 1);
    emu.regs_mut().rax = argc_addr;
}
